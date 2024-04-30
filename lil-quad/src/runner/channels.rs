use std::sync::{Arc, Mutex};

use crossbeam_channel::{Receiver, Sender};
use lil_broker::{Database, Timestamp};

use crate::uav::UAV;

use super::UAVRunnerState;

pub enum UAVRunnerCommand {
    Start(Timestamp),
    Stop,
    TickStart(Timestamp),
    TickFinished(Timestamp),
}

pub struct UAVRunnerChannels {
    pub database_arc: Arc<Mutex<Database>>,
    pub command_channel: (Sender<UAVRunnerCommand>, Receiver<UAVRunnerCommand>),
    pub state_channel: (Sender<UAVRunnerState>, Receiver<UAVRunnerState>),
}

#[derive(Debug)]
pub struct UAVRunnerClientChannels {
    pub command_channel: Sender<UAVRunnerCommand>,
    pub state_channel: Receiver<UAVRunnerState>,
    pub database_arc: Arc<Mutex<Database>>,
}

impl UAVRunnerChannels {
    pub fn new(uav: &UAV) -> Self {
        let (command_tx, command_rx) = crossbeam_channel::unbounded();
        let (state_tx, state_rx) = crossbeam_channel::unbounded();
        UAVRunnerChannels {
            database_arc: uav.data.clone(),
            command_channel: (command_tx, command_rx),
            state_channel: (state_tx, state_rx),
        }
    }

    pub fn get_client_channels(&self) -> UAVRunnerClientChannels {
        UAVRunnerClientChannels {
            command_channel: self.command_channel.0.clone(),
            state_channel: self.state_channel.1.clone(),
            database_arc: self.database_arc.clone(),
        }
    }
}
