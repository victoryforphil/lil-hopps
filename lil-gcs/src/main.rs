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

use clap::Parser;
use victory_broker::adapters::tcp::TCPClientAdapter;
use victory_broker::adapters::tcp::TCPClientOptions;
use victory_broker::node::sub_callback::SubCallback;
use victory_broker::node::Node;
use victory_data_store::database::Datastore;
use victory_data_store::topics::TopicKey;
use websocket::sync::Server;
use websocket::OwnedMessage;

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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,

    #[clap(long, value_parser, help = "Command Hz ", default_value = "10.0")]
    hz: f32,

    #[clap(
        short,
        long,
        value_parser,
        help = "Duration in seconds",
        default_value = "100.0"
    )]
    duration: f32,

    #[clap(
        short,
        long,
        value_parser,
        help = "Arm time in seconds",
        default_value = "7.0"
    )]
    arm_time: f32,
}
pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
}

fn get_current_timestamp() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64()
}

impl SubCallback for TCPNodeSubscriber {
    fn on_update(&mut self, datapoints: &victory_data_store::datapoints::DatapointMap) {
        for (topic, datapoint) in datapoints.iter() {
            self.map
                .insert(topic.display_name(), format!("{:?}", datapoint.value));
        }

        // clear the console
        // print!("\x1b[2J\x1b[1;1H");
    }
}
fn main() {
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
    let mut node = Node::new("TCP Client".to_string(), client_handle, datastore.clone());

    let subscriber = TCPNodeSubscriber {
        map: BTreeMap::new(),
    };
    let subscriber_handle = Arc::new(Mutex::new(subscriber));

    let topic_key = TopicKey::from_str("");
    node.add_sub_callback(topic_key, subscriber_handle.clone());
    node.register();

    // let mut csv = File::create(".lil/gcs/latest.csv").unwrap();
    // New loop that prints the datapoints
    let subscriber_handle_clone = subscriber_handle.clone();
    thread::spawn(move || {
        let server = Server::bind("0.0.0.0:3030").expect("Failed to start WebSocket server");

        info!("Started web server at {}", "0.0.0.0:3030");

        for request in server.filter_map(Result::ok) {
            let subscriber_handle_clone = subscriber_handle_clone.clone();
            thread::spawn(move || {
                let client = request.accept().expect("Failed to accept connection");
                let (_, mut sender) = client.split().unwrap();

                loop {
                    thread::sleep(Duration::from_secs_f32(2.0));
                    let map = subscriber_handle_clone.lock().unwrap();

                    let data = map
                        .map
                        .iter()
                        .map(|(topic, datapoint)| DataLine {
                            topic: topic.clone(),
                            datapoint: datapoint.clone(),
                        })
                        .collect::<Vec<DataLine>>();

                    let message = WebMessage {
                        timestamp: get_current_timestamp(),
                        data,
                    };

                    // HONESTLY - FUCK WHOEVER MADE to_vec and to_vec_nammed entirely fucking different.
                    let out = to_vec_named(&message).expect("Failed to serialize data");

                    info!("Websocket got new data {}", map.map.len());

                    if let Err(e) = sender.send_message(&OwnedMessage::Binary(out)) {
                        info!("Failed to send message: {:?}", e);
                        break;
                    }
                }
            });
        }
    });

    loop {
        thread::sleep(Duration::from_secs_f32(0.01));
        node.tick();
    }
}
