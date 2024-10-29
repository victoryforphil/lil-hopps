use std::collections::BTreeMap;
use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use lil_link::mavlink::core::QuadLinkCore;
use lil_link::mavlink::types::QuadMode;
use lil_link::systems::core::QuadlinkSystem;
use lil_quad::systems::timed_arm::TimedArm;
use lil_quad::systems::timed_mode::TimedMode;
use lil_quad::systems::timed_takeoff::TimedTakeoff;
use lil_rerun::system::RerunSystem;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use tracing_subscriber::layer::SubscriberExt;

use tracing_subscriber::FmtSubscriber;

use clap::{Parser, ValueEnum};
use victory_broker::adapters::tcp::TCPClientAdapter;
use victory_broker::adapters::tcp::TCPClientOptions;
use victory_broker::adapters::tcp::TCPServerAdapter;
use victory_broker::adapters::tcp::TCPServerOptions;
use victory_broker::node::sub_callback::SubCallback;
use victory_broker::node::Node;
use victory_commander::system::runner::BasherSysRunner;
use victory_data_store::database::Datastore;
use victory_data_store::datapoints::DatapointMap;
use victory_data_store::topics::TopicKey;
use victory_wtf::Timepoint;
use victory_wtf::Timespan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,

    #[clap(long, value_parser, help = "Command Hz ", default_value = "10.0")]
    hz: f32,

    #[clap(short, long, value_parser, help = "Duration in seconds", default_value = "100.0")]
    duration: f32,

    #[clap(short, long, value_parser, help = "Arm time in seconds", default_value = "7.0")]
    arm_time: f32,
}
pub struct TCPNodeSubscriber {
    map: BTreeMap<String, String>,
}

impl SubCallback for TCPNodeSubscriber {
    fn on_update(&mut self, datapoints: &victory_data_store::datapoints::DatapointMap) {
       
        for (topic, datapoint) in datapoints.iter() {
            self.map.insert(topic.display_name(), format!("{:?}", datapoint.value));
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
 

        let subscriber = TCPNodeSubscriber {map: BTreeMap::new()    };
        let subscriber_handle = Arc::new(Mutex::new(subscriber));
        
        let topic_key = TopicKey::from_str("");
        node.add_sub_callback(topic_key, subscriber_handle.clone());
        node.register();

        let mut csv = File::create(".lil/gcs/latest.csv").unwrap();
        // New loop that prints the datapoints
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs_f32(2.0));
                //clear the csv file
               
                let map = subscriber_handle.lock().unwrap();
              
                csv.rewind();
                for (topic, datapoint) in map.map.iter() {
                        // Save to CSV
                    let csv_string = format!("{},{}\n", topic, datapoint);
                   csv.write_all(csv_string.as_bytes()).unwrap();
                   
                }
               
            }
        }); 
    
        loop {
            thread::sleep(Duration::from_secs_f32(0.1));
            node.tick();
        }
    
    
}
