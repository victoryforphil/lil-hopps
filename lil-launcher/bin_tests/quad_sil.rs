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

use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

use clap::Parser;
use victory_broker::adapters::tcp::TCPServerAdapter;
use victory_broker::adapters::tcp::TCPServerOptions;
use victory_commander::system::runner::BasherSysRunner;
use victory_data_store::primitives::Primitives;
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

    #[clap(
        short,
        long,
        value_parser,
        help = "Arm time in seconds",
        default_value = "7.0"
    )]
    arm_time: f32,
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
    info!("Running 'quad_sil' with args: {:#?}", args);

    let mut runner = BasherSysRunner::new();
    let server = TCPServerAdapter::new(TCPServerOptions {
        port: 7001,
        address: "0.0.0.0".to_string(),
        update_interval: Timespan::new_hz(100.0),
    });
    let server_handle = Arc::new(Mutex::new(server));
    runner.enable_pubsub(server_handle);
    runner.dt = Timespan::new_hz(args.hz as f64);

    runner.add_system(Arc::new(Mutex::new(
        QuadlinkSystem::new_from_connection_string(args.connection_string.as_str()).unwrap(),
    )));

    let arm_task = ConditionTask::new(
        "arm".to_string(),
        TopicKey::from_str(&format!(
            "{}/{}/healthy",
            IDENT_BASE_STATUS, IDENT_STATUS_HEALTH
        )),
        Some(Primitives::Boolean(true)),
        Tasks::Arm,
    );

    let mode_task = TimedTask::new(
        "mode".to_string(),
        Timespan::new_secs(2.0),
        Tasks::SetMode(QuadMode::Guided),
    );

    let takeoff_task = TimedTask::new(
        "takeoff".to_string(),
        Timespan::new_secs(2.0),
        Tasks::Takeoff(11.0),
    );

    let waypoint_task = TimedTask::new(
        "waypoint".to_string(),
        Timespan::new_secs(10.0),
        Tasks::Waypoint(QuadPoseNED::new_xyz(0.0, 5.0, -10.0)),
    );
    let waypoint_2_task = TimedTask::new(
        "waypoint_2".to_string(),
        Timespan::new_secs(10.0),
        Tasks::Waypoint(QuadPoseNED::new_xyz(0.0, 0.0, -10.0)),
    );

    let land_task = TimedTask::new("land".to_string(), Timespan::new_secs(5.0), Tasks::Land);

    let mission = vec![
        TaskType::Condition(arm_task),
        TaskType::Timed(mode_task),
        TaskType::Timed(takeoff_task),
        TaskType::Timed(waypoint_task),
        TaskType::Timed(waypoint_2_task),
        TaskType::Timed(land_task),
    ];

    runner.add_system(Arc::new(Mutex::new(MissionRunner::new(mission))));

    runner.add_system(Arc::new(Mutex::new(HealthCheck::new(HealthCheckConfig {
        check_ekf: Some(true),
    }))));

    runner.add_system(Arc::new(Mutex::new(RerunSystem::new(
        "quad_arm".to_string(),
    ))));

    runner.set_real_time(true);
    runner.run();
}
