use std::sync::Arc;

use std::time::Duration;

use lil_link::common::identifiers::IDENT_BASE_STATUS;
use lil_link::common::identifiers::IDENT_STATUS_HEALTH;
use lil_link::common::types::mode::QuadMode;
use lil_link::common::types::pose_ned::QuadPoseNED;
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

    #[clap(long, help = "Enable tracing output")]
    tracing: bool,
}
use tracing_subscriber::layer::SubscriberExt;


#[tokio::main]
async fn main() {


    let args = SILArgs::parse();

    if !args.tracing {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(true)
            .pretty()
            .compact()
            .with_file(false)
            .with_line_number(false)
            .without_time()
            .init();
    } else {
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default())
        ).expect("setup tracy layer");
    }

    info!("Running 'quad_sil' with args: {:#?}", args);

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
    let node_info = BrokerNodeInfo::new("quad_sil_node");
    let mut node = BrokerNode::new(node_info, adapter_b);

    // Add systems as tasks
    let mavlink_sys = QuadlinkSystem::new_from_connection_string(args.connection_string.as_str()).unwrap();
    node.add_task(Arc::new(std::sync::Mutex::new(mavlink_sys))).unwrap();

    let health_check = HealthCheck::new(HealthCheckConfig {
        check_ekf: Some(true),
    });
    node.add_task(Arc::new(std::sync::Mutex::new(health_check))).unwrap();

    let stabilize_task = ConditionTask::new(
        "stabilize".to_string(),
        TopicKey::from_str(&format!(
            "{}/{}/healthy",
            IDENT_BASE_STATUS, IDENT_STATUS_HEALTH
        )),
        Some(victory_data_store::primitives::Primitives::Boolean(true)),
        Tasks::SetMode(QuadMode::Stabilize),
     
    );

    let arm_task = TimedTask::new(
        "arm".to_string(),
        Timespan::new_secs(3.0),
        Tasks::Arm,
    );
  
    let mode_task = TimedTask::new(
        "mode".to_string(),
        Timespan::new_secs(3.0),
        Tasks::SetMode(QuadMode::Guided),
    );

    let takeoff_task = TimedTask::new(
        "takeoff".to_string(),
        Timespan::new_secs(5.0),
        Tasks::Takeoff(2.0),
    );

    let waypoint_task = TimedTask::new(
        "waypoint".to_string(),
        Timespan::new_secs(5.0),
        Tasks::Waypoint(QuadPoseNED::new_xyz(1.0, 1.0, -2.0)),
    );

    let waypoint_2_task = TimedTask::new(
        "waypoint_2".to_string(),
        Timespan::new_secs(5.0),
        Tasks::Waypoint(QuadPoseNED::new_xyz(-1.0, -1.0, -2.0)),
    );

    let land_task = TimedTask::new("land".to_string(), Timespan::new_secs(5.0), Tasks::Land);

    let mission = vec![
        TaskType::Condition(stabilize_task),
        TaskType::Timed(arm_task),
        TaskType::Timed(mode_task),
        TaskType::Timed(takeoff_task),
        TaskType::Timed(waypoint_task),
        TaskType::Timed(waypoint_2_task),
        TaskType::Timed(land_task),
    ];

    let mission_runner = MissionRunner::new(mission);
    node.add_task(Arc::new(std::sync::Mutex::new(mission_runner))).unwrap();

    let rerun_sys = RerunSystem::new("quad_sil".to_string());
    if !args.tracing {
        node.add_task(Arc::new(std::sync::Mutex::new(rerun_sys))).unwrap();
    }

    // Initialize node
    node.init().unwrap();

    // Spawn node thread
    let node_handle = Arc::new(tokio::sync::Mutex::new(node));

    let node_thread = tokio::spawn(async move {
        loop {

            // Run node tick
            if let Err(e) = node_handle.try_lock().unwrap().tick() {
                warn!("Node // Error: {:?}", e);
            }

            // Sleep for 1ms to prevent busy loop
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
           
        }
    });

    let mut last_tick = Timepoint::now();
    let delay = Timespan::new_hz(args.hz as f64);
    loop {
        let tick_start = Timepoint::now();
        
        // Run broker tick
        if let Err(e) = broker.tick(delay.clone()).await {
            warn!("Broker // Error: {:?}", e);
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
