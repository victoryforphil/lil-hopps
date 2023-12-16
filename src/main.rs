use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use simulation::{runner::SimRunner, runner_options::SimRunnerOptions};
use viz::Visualization;

pub mod logging;
pub mod simulation;
pub mod types;
pub mod uav;
pub mod viz;
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
    let runner: SimRunner = SimRunner::new(SimRunnerOptions::new_unjoined(60.0));
    let runner_handle = Arc::new(Mutex::new(runner));
    let mut viz = Visualization::new(runner_handle);

    viz.init();
    viz.start();
}
