mod config;
mod detached;
mod state;
use std::{
    sync::{mpsc::Sender},
};

pub use config::*;
pub use detached::*;

use crate::{Simulation, SimulationState};

pub use state::*;
use tracing::info;

pub struct SimRunner {
    config: SimRunnerConfig,
    pub simulation: Simulation,
    pub state: SimulationState,
    pub runner_state: SimRunnerState,
}

impl SimRunner {
    pub fn new(config: SimRunnerConfig, simulation: Simulation) -> Self {
        Self {
            config,
            simulation,
            state: SimulationState::new(),
            runner_state: SimRunnerState::default(),
        }
    }

    pub fn init(&mut self) -> Result<(), anyhow::Error> {
        self.runner_state.uav_dbs = self.simulation.get_uav_databases();
        info!("Initializing simulation with uav dbs: {:?}", self.runner_state.uav_dbs.keys());
        self.simulation.init()?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        self.runner_state.state = SimRunnerStatus::Running;

        while self.runner_state.state == SimRunnerStatus::Running {
            let runner_state = &self.runner_state.clone();

            self.step(runner_state)?;

            self.runner_state.t = self.runner_state.t + self.config.dt;
            if self.runner_state.t >= self.config.max_t {
                self.runner_state.state = SimRunnerStatus::Completed;
                self.runner_state.uav_dbs = self.simulation.get_uav_databases();
                info!("Simulation completed")
            }
        }

        Ok(())
    }

    pub fn start_with_channel(&mut self, tx: Sender<SimRunnerUpdate>) -> Result<(), anyhow::Error> {
        self.runner_state.state = SimRunnerStatus::Running;
        info!("Starting simulation with channel");
        while self.runner_state.state == SimRunnerStatus::Running {
            let runner_state = &self.runner_state.clone();

            self.step(runner_state)?;
            self.runner_state.t = self.runner_state.t + self.config.dt;

            if self.runner_state.t >= self.config.max_t {
                self.shutdown()?;
                tx.send(SimRunnerUpdate {
                    state: self.runner_state.clone(),
                })
                .unwrap();
            }

            if self.runner_state.t.tick_ms % 1000 == 0 {
                self.runner_state.uav_dbs = self.simulation.get_uav_databases();
                tx.send(SimRunnerUpdate {
                    state: self.runner_state.clone(),
                })
                .unwrap();
            }
        }
        Ok(())
    }

    pub fn step(&mut self, runner_state: &SimRunnerState) -> Result<(), anyhow::Error> {
        self.simulation
            .step_physics(&runner_state.t, &self.config.dt)?;
        self.simulation
            .step_uavs(&runner_state.t, &self.config.dt)?;

        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), anyhow::Error> {
        self.runner_state.state = SimRunnerStatus::Completed;
        self.simulation.shutdown()?;
        info!("Simulation completed");
        Ok(())
    }
}
