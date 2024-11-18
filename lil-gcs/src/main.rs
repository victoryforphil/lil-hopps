use lil_link::common::types::mode::QuadMode;
use lil_link::common::types::parameter::QuadParameter;
use lil_link::common::types::request_arm::QuadSetModeRequest;
use lil_link::common::types::request_land::QuadLandRequest;
use lil_link::common::types::request_mode_set::QuadArmRequest;
use lil_link::common::types::request_takeoff::QuadTakeoffRequest;
use rmp_serde::to_vec_named;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use victory_data_store::database::listener::DataStoreListener;
use victory_data_store::sync::adapters::tcp::tcp_client::TCPClient;
use victory_data_store::sync::config::SyncConfig;

use clap::Parser;
use serde::Serialize;
use tracing::info;
use tracing::warn;
use tracing::Level;
use tracing_subscriber::fmt;

use victory_data_store::database::Datastore;
use victory_data_store::topics::TopicKey;
use victory_wtf::Timepoint;

use tokio::sync::{broadcast, mpsc};

mod webserver;

pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
    update: BTreeMap<String, String>,
}

impl DataStoreListener for TCPNodeSubscriber {
    fn on_datapoint(&mut self, datapoint: &victory_data_store::datapoints::Datapoint) {}

    fn on_raw_datapoint(&mut self, datapoint: &victory_data_store::datapoints::Datapoint) {
        let topic = datapoint.topic.clone();
        self.map
            .insert(topic.display_name(), format!("{:?}", datapoint.value));

        self.update
            .insert(topic.display_name(), format!("{:?}", datapoint.value));
    }

    fn on_bucket_update(&mut self, bucket: &victory_data_store::buckets::BucketHandle) {}
}

#[derive(Serialize, Deserialize, Debug)]
struct DataLine {
    topic: String,
    datapoint: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(
        short,
        long,
        value_parser,
        help = "Publishing connection string",
        default_value = "localhost:7001"
    )]
    connection: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WebMessage {
    timestamp: f64,
    data: Vec<DataLine>,
}

fn get_current_timestamp() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64()
}

/**
 * Parses Messages by Colon. The first Text portion is the command and the following X are the arguments.
 */
fn parse_message(message: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = message.splitn(2, ':').collect();

    let command = parts[0].to_string();
    let mut values = Vec::new();

    if parts.len() > 1 {
        values = parts[1].split(':').map(|s| s.to_string()).collect();
    }
    (command, values)
}

#[tokio::main]
async fn main() {
    let (tcp_tx, _) = broadcast::channel(512);
    let (ws_tx, mut ws_rx) = mpsc::channel(512);

    // Spawn WebSocket server task
    tokio::spawn(webserver::websocket_server_task(tcp_tx.clone(), ws_tx));

    fmt()
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    let args = SILArgs::parse();

    let mut client = TCPClient::new(args.connection.to_string()).await;

    while client.is_err() {
        info!("Failed to connect to server, retrying...");
        thread::sleep(Duration::from_secs_f32(1.0));
        client = TCPClient::new(args.connection.to_string()).await;
    }

    let client = client.unwrap();

    let client_handle = Arc::new(Mutex::new(client));
    let datastore = Datastore::new().handle();

    let topic_key = TopicKey::from_str("");

    let sync_config = SyncConfig {
        client_name: "GCS".to_string(),
        subscriptions: vec![topic_key.display_name()],
    };
    datastore
        .lock()
        .unwrap()
        .setup_sync(sync_config, client_handle);

    let listener = TCPNodeSubscriber {
        map: BTreeMap::new(),
        update: BTreeMap::new(),
    };

    let listener_handle = Arc::new(Mutex::new(listener));

    let _ = datastore
        .lock()
        .unwrap()
        .add_listener(&topic_key, listener_handle.clone());

    let subscriber_handle_clone = listener_handle.clone();
    let tcp_tx_clone = tcp_tx.clone();

    let subscriber_handle_clone_ws = listener_handle.clone();
    let tcp_tx_clone_ws = tcp_tx.clone();

    let datastore_clone = datastore.clone();
    tokio::spawn(async move {
        loop {
            if let Some(ws_msg) = ws_rx.recv().await {
                info!("WS MESSAGE -> {:#?}", ws_msg);

                // Need to parse from json or something a topic and a value. It would be great if I could parse a normal victory value.

                // I thought this was a scope thing at some point.
                let mut datastore = datastore_clone.lock().expect("Failed to lock mutex");

                let (msg_topic, params) = parse_message(&ws_msg);

                match msg_topic.as_str() {
                    "NEW_CLIENT" => {
                        // info!("GOT A NEW CLIENT YALL");

                        {
                            let mut map = subscriber_handle_clone_ws.lock().unwrap();

                            let data = map
                                .map
                                .iter()
                                .map(|(topic, datapoint)| DataLine {
                                    topic: topic.to_string(),
                                    datapoint: datapoint.to_string(),
                                })
                                .collect::<Vec<DataLine>>();

                            map.update.clear();

                            info!("Sending data? {0}", data.len());

                            let message = WebMessage {
                                timestamp: get_current_timestamp(),
                                data,
                            };

                            if let Ok(data_out) = to_vec_named(&message) {
                                let _ = tcp_tx_clone_ws.send(data_out); // Ignore if no clients are connected
                            } else {
                                warn!("Failed to MsgPack the DataStore Map")
                            }
                        }
                    }
                    "ARM" => {
                        info!("ARM the drone!");
                        let arm_request = QuadArmRequest::new(true);
                        datastore
                            .add_struct(
                                &TopicKey::from_str("cmd/arm"),
                                Timepoint::now(),
                                arm_request,
                            )
                            .unwrap();
                    }
                    "DISARM" => {
                        info!("Disarm the drone");
                        let arm_request = QuadArmRequest::new(false);
                        datastore
                            .add_struct(
                                &TopicKey::from_str("cmd/arm"),
                                Timepoint::now(),
                                arm_request,
                            )
                            .unwrap();
                    }
                    "TAKEOFF" => {
                        if params.len() == 1 {
                            if let Ok(set_val) = params[0].parse::<f32>() {
                                let mode_req = QuadTakeoffRequest::new(set_val);
                                datastore
                                    .add_struct(
                                        &TopicKey::from_str("cmd/takeoff"),
                                        Timepoint::now(),
                                        mode_req,
                                    )
                                    .unwrap();
                                info!("Takeoff requested at {0} feet", set_val);
                            }
                        } else {
                            warn!("Wrong number of commands sent to the MODE command");
                        }
                    }
                    "LAND" => {
                        info!("Land Requested");
                        let arm_request = QuadLandRequest::new();
                        datastore
                            .add_struct(
                                &TopicKey::from_str("cmd/land"),
                                Timepoint::now(),
                                arm_request,
                            )
                            .unwrap();
                    }
                    "MODE" => {
                        if params.len() == 1 {
                            if let Ok(mode) = QuadMode::from_str(&params[0]) {
                                println!("Setting new mode now {0}", mode);
                                let mode_req = QuadSetModeRequest::new(mode);
                                datastore
                                    .add_struct(
                                        &TopicKey::from_str("cmd/mode"),
                                        Timepoint::now(),
                                        mode_req,
                                    )
                                    .unwrap();
                            }
                        } else {
                            warn!("Wrong number of commands sent to the MODE command");
                        }
                    }
                    "PARAM" => {
                        if params.len() == 2 {
                            if let Ok(set_val) = params[1].parse::<f64>() {
                                let param_cmd = QuadParameter::new(params[0].clone(), set_val);
                                info!("Updated param {0} to {1}", params[0], set_val);
                                datastore
                                    .add_struct(
                                        &TopicKey::from_str(&params[0]),
                                        Timepoint::now(),
                                        param_cmd,
                                    )
                                    .unwrap();
                            } else {
                                warn!("Supplied incorrect data for Param Set -- Needs to be parsable as a float");
                            }
                        } else {
                            warn!("Wrong number of commands sent to the MODE command");
                        }
                    }
                    _ => {
                        warn!("Unknown command: {}", msg_topic);
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            // thread::sleep(Duration::from_secs_f32(2.0));
            tokio::time::sleep(Duration::from_millis(250)).await;
            {
                let mut map = subscriber_handle_clone.lock().unwrap();
                let updates = map.update.clone();
                map.update.clear();
                drop(map);
                if !updates.is_empty() {
                    let data = updates
                        .iter()
                        .map(|(topic, datapoint)| DataLine {
                            topic: topic.to_string(),
                            datapoint: datapoint.to_string(),
                        })
                        .collect::<Vec<DataLine>>();

                    let message = WebMessage {
                        timestamp: get_current_timestamp(),
                        data,
                    };

                    if let Ok(data_out) = to_vec_named(&message) {
                        let _ = tcp_tx_clone.send(data_out); // Ignore if no clients are connected
                    } else {
                        warn!("Failed to MsgPack the DataStore Map")
                    }
                }
            }
        }
    });
    let datastore = datastore.clone();
    loop {
        tokio::time::sleep(Duration::from_millis(50)).await;
        datastore.lock().unwrap().run_sync();
    }
}
