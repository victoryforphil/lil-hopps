use std::sync::Arc;
use std::sync::Mutex;

use lil_link::common::types::mode::QuadMode;
use lil_link::mavlink::system::QuadlinkSystem;
use lil_quad::systems::timed_arm::TimedArm;
use lil_quad::systems::timed_mode::TimedMode;
use lil_quad::systems::timed_takeoff::TimedTakeoff;
use lil_rerun::system::RerunSystem;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt;

use clap::Parser;
use victory_commander::system::runner::BasherSysRunner;
use victory_wtf::Timepoint;
use victory_wtf::Timespan;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ArmArgs {
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

    let args = ArmArgs::parse();
    info!("Running 'link_arm' with args: {:#?}", args);

    let mut runner = BasherSysRunner::new();
    runner.dt = Timespan::new_hz(args.hz as f64);

    runner.add_system(Arc::new(Mutex::new(
        QuadlinkSystem::new_from_connection_string(args.connection_string.as_str()).unwrap(),
    )));

    runner.add_system(Arc::new(Mutex::new(TimedArm::new(Timepoint::new_secs(
        args.arm_time as f64,
    )))));

    runner.add_system(Arc::new(Mutex::new(TimedMode::new(
        Timepoint::new_secs(args.arm_time as f64 + 1.0),
        QuadMode::Stabilize,
    ))));
    runner.add_system(Arc::new(Mutex::new(TimedMode::new(
        Timepoint::new_secs(args.arm_time as f64 + 4.0),
        QuadMode::Guided,
    ))));

    runner.add_system(Arc::new(Mutex::new(TimedTakeoff::new(
        Timepoint::new_secs(args.arm_time as f64 + 8.0),
        11.0,
    ))));
    runner.add_system(Arc::new(Mutex::new(RerunSystem::new(
        "quad_arm".to_string(),
    ))));

    runner.set_real_time(true);
    runner.run(Timepoint::new_secs(args.duration as f64));
    let mut keys = runner.data_store.lock().unwrap().get_all_keys();
    keys.sort_by(|a: &Arc<victory_data_store::topics::TopicKey>, b| {
        a.display_name().cmp(&b.display_name())
    });
    for key in keys {
        let latest = runner
            .data_store
            .lock()
            .unwrap()
            .get_latest_primitive(&key)
            .unwrap();
        info!(" {:?} \t\t {:?}", key, latest);
    }
}
