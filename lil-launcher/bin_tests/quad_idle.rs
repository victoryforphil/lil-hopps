use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use lil_link::common::identifiers::IDENT_BASE_STATUS;
use lil_link::common::identifiers::IDENT_STATUS_HEALTH;
use lil_link::common::types::mode::QuadMode;
use lil_link::common::types::pose_ned::QuadPoseNED;
use lil_link::mavlink::system::QuadlinkSystem;
use lil_quad::systems::health_check::HealthCheck;
use lil_quad::systems::health_check::HealthCheckConfig;
use lil_rerun::system::RerunSystem;
use tracing::info;
use tracing::warn;
use tracing::Level;
use tracing_subscriber::fmt;

use clap::Parser;
use victory_broker::adapters::tcp::tcp_server::TcpBrokerServer;
use victory_broker::broker::Broker;
use victory_broker::commander::linear::LinearBrokerCommander;
use victory_broker::node::info::BrokerNodeInfo;
use victory_broker::node::BrokerNode;
use victory_data_store::topics::TopicKey;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,

    #[clap(short, long, default_value = "0.0.0.0")]
    address: String,

    #[clap(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .pretty()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    let args = SILArgs::parse();
    info!("Running 'quad_idle' with args: {:#?}", args);

    let bind_addr = format!("{}:{}", args.address, args.port);
    info!("Broker Test TCP Server // Binding to {}", bind_addr);

    // Create broker with TCP server
    let server = TcpBrokerServer::new(&bind_addr).await.unwrap();
    let mut broker = Broker::new(LinearBrokerCommander::new());
    broker.add_adapter(Arc::new(Mutex::new(server)));

    // Create channel adapter pair for local node
    let (adapter_a, adapter_b) = victory_broker::adapters::channel::ChannelBrokerAdapter::new_pair();
    broker.add_adapter(adapter_a);

    // Create local node
    let node_info = BrokerNodeInfo::new("quad_idle_node");
    let mut node = BrokerNode::new(node_info, adapter_b);

    // Add systems as tasks
    let mavlink_sys = QuadlinkSystem::new_from_connection_string(args.connection_string.as_str()).unwrap();
    node.add_task(Arc::new(Mutex::new(mavlink_sys))).unwrap();

    let health_check = HealthCheck::new(HealthCheckConfig {
        check_ekf: Some(true),
    });
    node.add_task(Arc::new(Mutex::new(health_check))).unwrap();

    let rerun_sys = RerunSystem::new("quad_idle".to_string());
    node.add_task(Arc::new(Mutex::new(rerun_sys))).unwrap();

    // Initialize node
    node.init().unwrap();

    // Spawn node thread
    let node_handle = Arc::new(Mutex::new(node));
    let node_thread = tokio::spawn(async move {
        loop {
            {
                let mut node = node_handle.lock().unwrap();
                if let Err(e) = node.tick() {
                    warn!("Node // Error: {:?}", e);
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    });

    // Main broker loop
    loop {
        match broker.tick() {
            Ok(_) => (),
            Err(e) => {
                warn!("Broker // Error: {:?}", e);
            }
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}
