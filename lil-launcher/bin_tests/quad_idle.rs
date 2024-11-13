use std::sync::Arc;
use std::sync::Mutex;

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
use lil_quad::systems::timed_arm::TimedArm;
use lil_quad::systems::timed_mode::TimedMode;
use lil_quad::systems::timed_takeoff::TimedTakeoff;
use lil_rerun::system::RerunSystem;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

use clap::Parser;
use victory_broker::adapters::tcp::TCPServerAdapter;
use victory_broker::adapters::tcp::TCPServerOptions;
use victory_commander::system::runner::BasherSysRunner;
use victory_data_store::database::retention::RetentionPolicy;
use victory_data_store::primitives::Primitives;
use victory_data_store::sync::adapters::tcp::tcp_server::TcpSyncServer;
use victory_data_store::sync::config::SyncConfig;
use victory_data_store::topics::TopicKey;
use victory_wtf::Timepoint;
use victory_wtf::Timespan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct SILArgs {
    #[clap(short, long, value_parser, help = "Mavlink connection string")]
    connection_string: String,

    #[clap(long, value_parser, help = "Command Hz ", default_value = "25.0")]
    hz: f32,

    #[clap(
        short,
        long,
        value_parser,
        help = "Duration in seconds",
        default_value = "100.0"
    )]
    duration: f32,

    #[clap(short, long, value_parser, help = "TCP Sync Server address")]
    sync_server_address: String,

    #[clap(
        short,
        long,
        value_parser,
        help = "Arm time in seconds",
        default_value = "7.0"
    )]
    arm_time: f32,
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
    info!("Running 'quad_sil' with args: {:#?}", args);

    let mut runner = BasherSysRunner::new();
    let server = TcpSyncServer::new(&args.sync_server_address).await;
    let server_handle = Arc::new(Mutex::new(server));

    let topic_filter = TopicKey::from_str("cmd");

    let sync_config = SyncConfig {
        client_name: "Quad Idle".to_string(),
        subscriptions: vec![topic_filter.display_name()],
    };
    runner
        .data_store
        .lock()
        .unwrap()
        .setup_sync(sync_config, server_handle);

    let retention_policy = RetentionPolicy {
        max_age: Some(Timespan::new_secs(30.0)),
        max_rows: Some(64),
    };
    runner.data_store.lock().unwrap().set_retention(retention_policy);

    runner.dt = Timespan::new_hz(args.hz as f64);
    runner.set_end_time(Timepoint::new_secs(60. * 15.));


    runner.add_system(Arc::new(Mutex::new(
        QuadlinkSystem::new_from_connection_string(args.connection_string.as_str()).unwrap(),
    )));

    runner.add_system(Arc::new(Mutex::new(HealthCheck::new(HealthCheckConfig {
        check_ekf: Some(true),
    }))));

    runner.add_system(Arc::new(Mutex::new(RerunSystem::new(
        "quad_idle".to_string(),
    ))));

    runner.set_real_time(true);
    runner.run();
}
