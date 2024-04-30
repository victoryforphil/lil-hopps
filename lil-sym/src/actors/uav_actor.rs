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
    pub state: UAVActorState,
}

impl UAVActor {
    pub fn new(runner_handle: UAVThreadedRunner) -> Self {
        UAVActor {
            uav_runner: runner_handle,
            state: UAVActorState::new(),
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
        self.state = UAVActorState {
            uav_channels: Some(channels),
        };
        Ok(self.state.clone())
    }

    fn step(
        &mut self,
        context: crate::SimContextHandle,
        state: &crate::SimulationState,
        t: &lil_broker::Timestamp,
        dt: &lil_broker::Timestamp,
    ) -> Result<UAVActorState, anyhow::Error> {
        if self.state.uav_channels.is_none() {
            return Ok(self.state.clone());
        }

        let mut channels = self.state.uav_channels.as_ref().unwrap().clone();
        channels
            .command_channel
            .send(UAVRunnerCommand::TickStart(t.clone()))
            .unwrap();

        Ok(self.state.clone())
    }
}
