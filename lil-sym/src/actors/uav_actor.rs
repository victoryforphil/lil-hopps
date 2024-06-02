

use lil_broker::Timestamp;
use lil_quad::{
    runner::{UAVRunnerClientChannels, UAVRunnerCommand, UAVThreadedRunner},
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

    pub fn shutdown(&mut self) {
        if let Some(channels) = &self.state.uav_channels {
            channels
                .command_channel
                .send(UAVRunnerCommand::Stop)
                .unwrap();
        }
    }
}

impl SimActor<UAVActorState> for UAVActor {
    fn init(
        &mut self,
        _context: crate::SimContextHandle,
        _last_state: &crate::SimulationState,
    ) -> Result<UAVActorState, anyhow::Error> {
        let channels = self.uav_runner.start()?;
        channels
            .command_channel
            .send(UAVRunnerCommand::Start(Timestamp::from_seconds(10000.0)))
            .unwrap();
        self.state = UAVActorState {
            uav_channels: Some(channels),
        };
        Ok(self.state.clone())
    }

    fn step(
        &mut self,
        _context: crate::SimContextHandle,
        _state: &crate::SimulationState,
        t: &lil_broker::Timestamp,
        _dt: &lil_broker::Timestamp,
    ) -> Result<UAVActorState, anyhow::Error> {
        if self.state.uav_channels.is_none() {
            return Ok(self.state.clone());
        }

        let channels = self.state.uav_channels.as_ref().unwrap().clone();
        channels
            .command_channel
            .send(UAVRunnerCommand::TickStart(t.clone()))
            .unwrap();

        Ok(self.state.clone())
    }
}
