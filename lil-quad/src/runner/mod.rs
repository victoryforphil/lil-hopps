mod channels;
mod config;
mod detached;
mod state;
use std::sync::mpsc::Sender;

pub use channels::*;
pub use config::*;
pub use detached::*;
pub use state::*;
use tracing::{debug, info};

use crate::uav::UAV;

pub struct UAVRunner {
    pub config: UAVRunnerConfig,
    pub uav: UAV,
    pub runner_state: UAVRunnerState,
    pub channels: UAVRunnerChannels,
}

impl UAVRunner {
    pub fn new(config: UAVRunnerConfig, uav: UAV) -> Self {
        let chans = UAVRunnerChannels::new(&uav);
        Self {
            config,
            uav: uav,
            runner_state: UAVRunnerState::default(),
            channels: chans,
        }
    }

    pub fn init(&mut self) -> Result<(), anyhow::Error> {
        self.channels.database_arc = self.uav.data.clone();
        self.runner_state.state = UAVRunnerStatus::Init;
        Ok(())
    }

    pub fn start(&mut self) -> Result<UAVRunnerState, anyhow::Error> {
        while self.runner_state.state != UAVRunnerStatus::Completed {
            let incoming_command = self.channels.command_channel.1.try_recv();

            if incoming_command.is_ok() {
                match incoming_command.unwrap() {
                    UAVRunnerCommand::Start(max_t) => {
                        info!("Received start command with max_t: {:?}", max_t);
                        self.runner_state.state = UAVRunnerStatus::Running;
                        self.config.max_t = max_t;
                    }
                    UAVRunnerCommand::Stop => {
                        info!("Received stop command");
                        self.runner_state.state = UAVRunnerStatus::Completed;
                    }
                    UAVRunnerCommand::TickStart(t) => {
                        debug!("Received tick start command at t: {:?}", t);
                        self.runner_state.t = t;
                        let runner_state = &self.runner_state.clone();

                        self.step(runner_state)?;
                        self.runner_state.ticks += 1;
                    }
                    UAVRunnerCommand::TickFinished(_) => todo!(),
                }
            }

            if !self.config.external_tick && self.runner_state.state == UAVRunnerStatus::Running {
                let runner_state = &self.runner_state.clone();

                self.step(runner_state)?;

                self.runner_state.t = self.runner_state.t + self.config.dt;
                self.runner_state.ticks += 1;

                self.channels
                    .state_channel
                    .0
                    .send(self.runner_state.clone())
                    .unwrap();
            }

            if self.runner_state.t >= self.config.max_t ||(!self.config.wait && self.runner_state.state == UAVRunnerStatus::Init) {
                self.runner_state.state = UAVRunnerStatus::Completed;
                info!("UAV Runner completed")
            }

        }

        Ok(self.runner_state.clone())
    }

    pub fn step(&mut self, runner_state: &UAVRunnerState) -> Result<UAVRunnerState, anyhow::Error> {
        self.uav.tick(&runner_state.t)?;
        debug!("UAV ticked at t: {:?}", runner_state.t);
        Ok(self.runner_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::uav::{MockUAVRuntime, UAVRuntime};

    use super::*;
    use lil_broker::Timestamp;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_uav_runner_blocking_started() {
        env_logger::init();
        let uav_runtime = MockUAVRuntime::new().as_arc_mutex();
        let uav = UAV::new(uav_runtime);
        let config = UAVRunnerConfig::default().set_wait();
        let mut runner = UAVRunner::new(config.clone(), uav);
        runner.init().unwrap();

        runner
            .channels
            .command_channel
            .0
            .send(UAVRunnerCommand::Start(Timestamp::from_seconds(5.0)))
            .unwrap();

        let final_state = runner.start().unwrap();

        let expected_ticks = config.max_t.seconds() * (1.0 / config.dt.seconds());
        assert_eq!(final_state.ticks, expected_ticks as u64);
        assert_eq!(final_state.state, UAVRunnerStatus::Completed);
        assert_eq!(final_state.t, config.max_t);
    }

    #[test]
    pub fn test_uav_runner_blocking_not_started() {
        env_logger::init();
        let uav_runtime = MockUAVRuntime::new().as_arc_mutex();
        let uav = UAV::new(uav_runtime);
        let config = UAVRunnerConfig::default();
        let mut runner = UAVRunner::new(config.clone(), uav);
        runner.init().unwrap();

        let final_state = runner.start().unwrap();

        let expected_ticks = 0;
        assert_eq!(final_state.ticks, expected_ticks as u64);
        assert_eq!(final_state.state, UAVRunnerStatus::Completed);
        assert_eq!(final_state.t, Timestamp::zero());
    }

    #[test]
    pub fn test_uav_runner_blocking_external_tick() {
        env_logger::init();
        let uav_runtime = MockUAVRuntime::new().as_arc_mutex();
        let uav = UAV::new(uav_runtime);
        let config = UAVRunnerConfig::default().set_external_tick();
        let mut runner = UAVRunner::new(config.clone(), uav);
        runner.init().unwrap();

        runner
            .channels
            .command_channel
            .0
            .send(UAVRunnerCommand::Start(Timestamp::from_seconds(5.0)))
            .unwrap();

        runner
            .channels
            .command_channel
            .0
            .send(UAVRunnerCommand::TickStart(Timestamp::from_seconds(0.1)))
            .unwrap();
        runner
            .channels
            .command_channel
            .0
            .send(UAVRunnerCommand::Stop)
            .unwrap();

        let final_state = runner.start().unwrap();

        let expected_ticks = 1;
        assert_eq!(final_state.ticks, expected_ticks as u64);
        assert_eq!(final_state.state, UAVRunnerStatus::Completed);
        assert_eq!(final_state.t, Timestamp::from_seconds(0.1));
    }
}
