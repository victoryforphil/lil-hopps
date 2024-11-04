use rmp_serde::to_vec_named;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

use victory_broker::adapters::tcp::TCPClientAdapter;
use victory_broker::adapters::tcp::TCPClientOptions;
use victory_broker::node::sub_callback::SubCallback;
use victory_broker::node::Node;
use victory_data_store::database::Datastore;
use victory_data_store::topics::TopicKey;

use tokio::sync::{mpsc, broadcast};

mod webserver;

pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
    // update: Vec<(String, String )>
}

impl SubCallback for TCPNodeSubscriber {
    fn on_update(&mut self, datapoints: &victory_data_store::datapoints::DatapointMap) {
        for (topic, datapoint) in datapoints.iter() {
            self.map
                .insert(topic.display_name(), format!("{:?}", datapoint.value));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DataLine {
    topic: String,
    datapoint: String,
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

#[tokio::main]
async fn main() {
    let (tcp_tx, _) = broadcast::channel(4);
    let (ws_tx, mut ws_rx) = mpsc::channel(32);

    // Spawn WebSocket server task
    tokio::spawn(webserver::websocket_server_task(tcp_tx.clone(), ws_tx));

    fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    let mut client = TCPClientAdapter::new(TCPClientOptions::from_url("0.0.0.0:7001"));

    while client.is_err() {
        info!("Failed to connect to server, retrying...");
        thread::sleep(Duration::from_secs_f32(1.0));
        client = TCPClientAdapter::new(TCPClientOptions::from_url("0.0.0.0:7001"));
    }

    let client = client.unwrap();

    let client_handle = Arc::new(Mutex::new(client));
    let datastore = Datastore::new().handle();
    let mut node = Node::new("TCP Client".to_string(), client_handle.clone(), datastore.clone());

    let subscriber = TCPNodeSubscriber {
        map: BTreeMap::new(),
    };
    let subscriber_handle = Arc::new(Mutex::new(subscriber));

    let topic_key = TopicKey::from_str("");
    node.add_sub_callback(topic_key, subscriber_handle.clone());
    node.register();

    let subscriber_handle_clone = subscriber_handle.clone();
    let tcp_tx_clone = tcp_tx.clone();

    tokio::spawn(async move {
        tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(100));
                if let Some(ws_msg) = ws_rx.recv().await {
                    println!("{:#?}", ws_msg);
                }
            }
        });

        loop {
            thread::sleep(Duration::from_secs_f32(2.0));
            let map = subscriber_handle_clone.lock().unwrap();

            let data = map
                .map
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

            // HONESTLY - FUCK WHOEVER MADE to_vec and to_vec_nammed entirely fucking different.
            let out = to_vec_named(&message).expect("Failed to serialize data");

            info!("Sending data to webserver {}", map.map.len());
            let _ = tcp_tx_clone.send(out);
        }
    });

    loop {
        thread::sleep(Duration::from_millis(100));
        node.tick();
    }

}