use std::fs::File;

use log::{debug, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use simulation::{runner::SimRunner, runner_options::SimRunnerOptions};

pub mod simulation;
pub mod types;
pub mod uav;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("logs/lilhopps-main.log").unwrap(),
        ),
    ])
    .unwrap();
    let mut runner = SimRunner::new(SimRunnerOptions::new(3.0));
    runner.start();
}

#[test]
fn test_main() {
    main();
}
