use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use lil_link::common::identifiers::IDENT_BASE_STATUS;
use lil_link::common::identifiers::IDENT_STATUS_HEALTH;
use lil_link::common::identifiers::IDENT_STATUS_MODE;
use lil_link::common::types::mode::QuadMode;
use lil_link::common::types::pose_ned::QuadPoseNED;
use lil_link::common::types::request_led::QuadLedRequest;
use lil_link::mavlink::system::QuadlinkSystem;
use lil_quad::systems::health_check::HealthCheck;
use lil_quad::systems::health_check::HealthCheckConfig;
use lil_quad::systems::mission_runner::task::ConditionTask;
use lil_quad::systems::mission_runner::task::TaskType;
use lil_quad::systems::mission_runner::task::Tasks;
use lil_quad::systems::mission_runner::task::TimedTask;
use lil_quad::systems::mission_runner::MissionRunner;
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
use victory_data_store::primitives::Primitives;
use victory_data_store::topics::TopicKey;
use victory_wtf::Timepoint;
use victory_wtf::Timespan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,

    #[clap(long, value_parser, help = "Command Hz ", default_value = "50.0")]
    hz: f32,

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
    broker.add_adapter(Arc::new(tokio::sync::Mutex::new(server)));

    // Create channel adapter pair for local node
    let (adapter_a, adapter_b) = victory_broker::adapters::channel::ChannelBrokerAdapter::new_pair();
    broker.add_adapter(adapter_a);

    // Create local node
    let node_info = BrokerNodeInfo::new("quad_idle_node");
    let mut node = BrokerNode::new(node_info, adapter_b);

        // Create LED tasks
        let initial_red = TaskType::Timed(
            TimedTask::new(
                "Initial Red".to_string(),
                Timespan::new_secs(0.),
                Tasks::Led(QuadLedRequest::new(255, 0, 0))
            )
        );
    
        let white_when_healthy = TaskType::Condition(
            ConditionTask::new(
                "White When Healthy".to_string(), 
                TopicKey::from_str(&format!(
                    "{}/{}/healthy",
                    IDENT_BASE_STATUS, IDENT_STATUS_HEALTH
                )),
                Some(Primitives::Boolean(true)),
                Tasks::Led(QuadLedRequest::new(255, 255, 255))
            )
        );
    
        let red_when_flying = TaskType::Condition(
            ConditionTask::new(
                "Red When Flying".to_string(),
                TopicKey::from_str(&format!(
                    "{}/{}/mode",
                    IDENT_BASE_STATUS, IDENT_STATUS_MODE
                )), 
                Some(Primitives::Text("GUIDED".to_string())),
                Tasks::Led(QuadLedRequest::new(255, 0, 0))
            )
        );
    
    let mission_runner = MissionRunner::new(vec![initial_red, white_when_healthy, red_when_flying]);
    node.add_task(Arc::new(Mutex::new(mission_runner))).unwrap();

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
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });

    let mut last_tick = Timepoint::now();
    let delay = Timespan::new_hz(args.hz as f64);
    
    // Main broker loop
    loop {
        let tick_start = Timepoint::now();
        
        match broker.tick(delay.clone()).await {
            Ok(_) => (),
            Err(e) => {
                warn!("Broker // Error: {:?}", e);
            }
        }

        // Sleep for remaining time
        let tick_duration = Timepoint::now() - tick_start.clone();
        let sleep_duration = delay.as_duration();
        if tick_duration.as_duration() > sleep_duration {
            warn!(
                "Broker thread running slower than target rate by {:.2?} ms",
                (tick_duration.as_duration().as_millis() - sleep_duration.as_millis())
            );
        }
        tokio::time::sleep(sleep_duration.saturating_sub(tick_duration.as_duration())).await;
    }
}
