use std::sync::{Arc, Mutex};

use lil_broker::Timestamp;
use lil_quad::{
    runner::{UAVRunner, UAVRunnerClientChannels, UAVRunnerCommand, UAVThreadedRunner},
    uav::{UAVRuntime, UAV},
};

use crate::SimActor;

#[derive(Debug, Clone, Default)]
pub struct UAVActorState {
    pub uav_channels: Option<UAVRunnerClientChannels>,
}

impl UAVActorState {
    pub fn new() -> Self {
        Self { uav_channels: None }
    }
}

pub struct UAVActor {
    pub uav_runner: UAVThreadedRunner,
}

impl UAVActor {
    pub fn new(runner_handle: UAVThreadedRunner) -> Self {
        UAVActor {
            uav_runner: runner_handle,
        }
    }
}

impl SimActor<UAVActorState> for UAVActor {
    fn init(
        &mut self,
        context: crate::SimContextHandle,
        last_state: &crate::SimulationState,
    ) -> Result<UAVActorState, anyhow::Error> {
        let mut channels = self.uav_runner.start()?;
        channels
            .command_channel
            .send(UAVRunnerCommand::Start(Timestamp::from_seconds(50.0)))
            .unwrap();
        Ok(UAVActorState {
            uav_channels: Some(channels),
        })
    }

    fn step(
        &mut self,
        context: crate::SimContextHandle,
        state: &crate::SimulationState,
        t: &lil_broker::Timestamp,
        dt: &lil_broker::Timestamp,
    ) -> Result<UAVActorState, anyhow::Error> {
        self.st
            .command_channel
            .0
            .send(UAVRunnerCommand::TickStart(t.clone()))
            .unwrap();

        Ok(UAVActorState::new())
    }
}
