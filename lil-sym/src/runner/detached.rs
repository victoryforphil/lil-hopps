use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use lil_broker::Database;
use tracing::info;

use crate::{Scenario, SimRunner, SimRunnerConfig, SimRunnerState, Simulation};

#[derive(Debug, Clone)]
pub struct SimRunnerUpdate {
    pub state: SimRunnerState,
}

#[derive(Debug, Clone)]
pub struct SimRunnerResult {
    pub state: SimRunnerState,
}

pub struct SimThreadedRunner {
    scenario: Arc<dyn Scenario>,
    config: SimRunnerConfig,
    thread: Option<JoinHandle<SimRunnerResult>>,
}

impl SimThreadedRunner {
    pub fn new(scenario: Arc<dyn Scenario>, config: SimRunnerConfig) -> Self {
        Self {
            scenario,
            config,
            thread: None,
        }
    }

    pub fn start(&mut self) -> Result<Receiver<SimRunnerUpdate>, anyhow::Error> {
        let scenario = self.scenario.clone();
        let config = self.config.clone();

        let (tx, rx): (Sender<SimRunnerUpdate>, Receiver<SimRunnerUpdate>) = mpsc::channel();

        let thread = thread::spawn(move || {
            info!("Starting simulation in thread");
            let sim = Simulation::new(scenario.as_ref());
            let mut runner = SimRunner::new(config, sim);
            runner.init().unwrap();
            runner.start_with_channel(tx).unwrap();
            SimRunnerResult {
                state: runner.runner_state.clone(),
            }
        });
        self.thread = Some(thread);

        Ok(rx)
    }
}
