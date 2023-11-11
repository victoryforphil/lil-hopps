use std::{fs::File};

use log::{LevelFilter, debug};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config, TerminalMode, ColorChoice};
use simulation::{runner::SimRunner, runner_options::SimRunnerOptions};

pub mod types;
pub mod uav;
pub mod simulation;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    let mut runner = SimRunner::new(SimRunnerOptions::new(3.0));
    runner.start();

    while runner.channel_rx.recv().unwrap().running {
        debug!("Main thread tick");
        //thread::sleep(Duration::from_millis(500));
    }
}
