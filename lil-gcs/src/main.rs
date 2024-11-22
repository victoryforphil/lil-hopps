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
use victory_broker::adapters::tcp::tcp_client::TcpBrokerClient;
use victory_broker::node::info::BrokerNodeInfo;
use victory_broker::node::BrokerNode;
use victory_broker::task::config::BrokerCommanderFlags;
use victory_broker::task::config::BrokerTaskConfig;
use victory_broker::task::subscription::BrokerTaskSubscription;
use victory_broker::task::trigger::BrokerTaskTrigger;
use victory_broker::task::BrokerTask;
use victory_data_store::database::listener::DataStoreListener;
use victory_data_store::database::view::DataView;

use victory_wtf::Timespan;

use clap::Parser;
use serde::Serialize;
use tracing::info;
use tracing::warn;
use tracing::Level;
use tracing_subscriber::fmt;

use victory_data_store::topics::TopicKey;

use tokio::sync::{broadcast, mpsc};

mod webserver;

#[derive(Clone)]
pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
    update: BTreeMap<String, String>,
    command_queue: Arc<Mutex<DataView>>,
}

impl BrokerTask for TCPNodeSubscriber {
    fn get_config(&self) -> victory_broker::task::config::BrokerTaskConfig {
        BrokerTaskConfig::new("tcp-node-subscriber")
            .with_trigger(BrokerTaskTrigger::Rate(Timespan::new_ms(20.0)))
            .with_subscription(BrokerTaskSubscription::new_latest(&TopicKey::from_str("status")))
            .with_subscription(BrokerTaskSubscription::new_latest(&TopicKey::from_str("cmd")))
            .with_subscription(BrokerTaskSubscription::new_latest(&TopicKey::from_str("log")))
            .with_subscription(BrokerTaskSubscription::new_latest(&TopicKey::from_str("pose")))
            .with_flag(BrokerCommanderFlags::NonBlocking)
    }

    fn on_execute(
        &mut self,
        inputs: &victory_data_store::database::view::DataView,
    ) -> Result<victory_data_store::database::view::DataView, anyhow::Error> {
        let data_map = inputs.maps.clone();
        for (topic, value) in data_map.iter() {
            self.map
                .insert(topic.display_name(), format!("{:?}", value));

            self.update
                .insert(topic.display_name(), format!("{:?}", value));
        }

        let output = if let Ok(queue) = self.command_queue.lock() {
            queue.clone()
        } else {
            DataView::new()
        };

        Ok(output)
    }
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

    let mut client = TcpBrokerClient::new(&args.connection).await;

    while client.is_err() {
        info!("Failed to connect to server, retrying...");
        thread::sleep(Duration::from_secs_f32(1.0));
        client = TcpBrokerClient::new(&args.connection).await;
    }

    let client = client.unwrap();

    let client_handle = Arc::new(Mutex::new(client));

    let command_queue = Arc::new(Mutex::new(DataView::new()));
    let sub_task = TCPNodeSubscriber {
        map: BTreeMap::new(),
        update: BTreeMap::new(),
        command_queue: command_queue.clone(),
    };
    let sub_task_handle = Arc::new(Mutex::new(sub_task));

    let node_info = BrokerNodeInfo::new("tcp-node-subscriber");
    let mut node = BrokerNode::new(node_info, client_handle);
    node.add_task(sub_task_handle.clone()).unwrap();

    let sub_task_handle_clone = sub_task_handle.clone();
    let tcp_tx_clone = tcp_tx.clone();

    let subscriber_handle_clone_ws = sub_task_handle_clone.clone();
    let tcp_tx_clone_ws = tcp_tx.clone();

    // Sending commands to the drone
    tokio::spawn(async move {
        loop {
            if let Some(ws_msg) = ws_rx.recv().await {
                info!("WS MESSAGE -> {:#?}", ws_msg);

                let (msg_topic, params) = parse_message(&ws_msg);

                match msg_topic.as_str() {
                    "NEW_CLIENT" => {
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
                        if let Ok(mut queue) = command_queue.lock() {
                            queue
                                .add_latest(&TopicKey::from_str("cmd/arm"), arm_request)
                                .expect("Failed to add datapoint to queue");
                        }
                    }
                    "DISARM" => {
                        info!("Disarm the drone");
                        let arm_request = QuadArmRequest::new(false);
                        if let Ok(mut queue) = command_queue.lock() {
                            queue
                                .add_latest(&TopicKey::from_str("cmd/arm"), arm_request)
                                .expect("Failed to add datapoint to queue");
                        }
                    }
                    "TAKEOFF" => {
                        if params.len() == 1 {
                            if let Ok(set_val) = params[0].parse::<f32>() {
                                let mode_req = QuadTakeoffRequest::new(set_val);
                                if let Ok(mut queue) = command_queue.lock() {
                                    queue
                                        .add_latest(&TopicKey::from_str("cmd/takeoff"), mode_req)
                                        .expect("Failed to add datapoint to queue");
                                }
                                info!("Takeoff requested at {0} feet", set_val);
                            }
                        } else {
                            warn!("Wrong number of commands sent to the MODE command");
                        }
                    }
                    "LAND" => {
                        info!("Land Requested");
                        let arm_request = QuadLandRequest::new();
                        if let Ok(mut queue) = command_queue.lock() {
                            queue
                                .add_latest(&TopicKey::from_str("cmd/land"), arm_request)
                                .expect("Failed to add datapoint to queue");
                        }
                    }
                    "MODE" => {
                        if params.len() == 1 {
                            if let Ok(mode) = QuadMode::from_str(&params[0]) {
                                println!("Setting new mode now {0}", mode);
                                let mode_req = QuadSetModeRequest::new(mode);
                                if let Ok(mut queue) = command_queue.lock() {
                                    queue
                                        .add_latest(&TopicKey::from_str("cmd/mode"), mode_req)
                                        .expect("Failed to add datapoint to queue");
                                }
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
                                if let Ok(mut queue) = command_queue.lock() {
                                    queue
                                        .add_latest(&TopicKey::from_str(&params[0]), param_cmd)
                                        .expect("Failed to add datapoint to queue");
                                }
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
                let mut map = sub_task_handle.lock().unwrap();
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

    node.init().unwrap();
    loop {
        tokio::time::sleep(Duration::from_millis(50)).await;
        match node.tick() {
            Ok(_) => (),
            Err(e) => {
                warn!("Error in node tick: {e}");
            }
        }
    }
}
