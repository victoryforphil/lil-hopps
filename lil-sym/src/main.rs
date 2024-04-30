use std::sync::Arc;

use lil_sym::{DefaultScenario, SimRunnerConfig, SimThreadedRunner};

fn main() {
    let mut scenario = DefaultScenario {};

    let mut sim_config = SimRunnerConfig::default();
    let mut sim_runner = SimThreadedRunner::new(Arc::new(scenario), sim_config);
}
