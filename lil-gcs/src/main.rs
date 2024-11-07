use std::collections::BTreeMap;
use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use lil_link::common::types::request_mode_set::QuadArmRequest;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use clap::{Parser, ValueEnum};
use victory_broker::adapters::tcp::TCPClientAdapter;
use victory_broker::adapters::tcp::TCPClientOptions;
use victory_broker::node::sub_callback::SubCallback;
use victory_broker::node::Node;
use victory_data_store::database::Datastore;
use victory_data_store::topics::TopicKey;
use victory_wtf::Timepoint;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Publishing connection string")]
    connection: String,
}
pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
}

impl SubCallback for TCPNodeSubscriber {
    fn on_update(&mut self, datapoints: &victory_data_store::datapoints::DatapointMap) {
        //  info!("Datapoints: {:?}", datapoints.len());
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
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    let args = SILArgs::parse();

    let mut client = TCPClientAdapter::new(TCPClientOptions::from_url(&args.connection));

    while client.is_err() {
        info!("Failed to connect to server, retrying...");
        thread::sleep(Duration::from_secs_f32(1.0));
        client = TCPClientAdapter::new(TCPClientOptions::from_url(&args.connection));
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

    let mut csv = File::create(".lil/gcs/latest.csv").unwrap();
    // New loop that prints the datapoints

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs_f32(2.0));
            // If ticke
            let map = subscriber_handle.lock().unwrap();

            csv.rewind();
            for (topic, datapoint) in map.map.iter() {
                // Save to CSV
                let csv_string = format!("{},{}\n", topic, datapoint);
                csv.write_all(csv_string.as_bytes()).unwrap();
            }

            info!("CSV updated with {} datapoints", map.map.len());
        }
    });
    let mut start_time = Timepoint::now();
    let mut fired = false;
    loop {
        thread::sleep(Duration::from_secs_f32(0.02));
        node.tick();

        /*
          let elapsed = Timepoint::now() - start_time.clone();
        if elapsed.secs() > 1.0 && !fired {
            fired = true;
            info!("Fired!");
            // Send arm command
            let arm_command = QuadArmRequest::new(true);
            let topic = TopicKey::from_str("cmd/arm");
            datastore
                .lock()
                .unwrap()
                .add_struct(&topic, Timepoint::now(), arm_command)
                .unwrap();
        } */
    }
}
