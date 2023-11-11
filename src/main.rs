use std::{fs::File};

use log::{LevelFilter, debug};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config, TerminalMode, ColorChoice};
use simulation::{runner_options::SimRunnerOptions, runner::SimRunner};

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

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::runner_options::SimRunnerOptions;

    /// TEST: test_sim_runner
    /// ----
    /// Expectations:
    /// - It runs.
    /// 
    /// Failures:
    /// - None
    #[test]
    fn test_sim_runner() {
        let mut runner = SimRunner::new(SimRunnerOptions::new(3.0));
        runner.start();
    }
}