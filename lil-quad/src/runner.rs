use std::sync::{mpsc::{Receiver, Sender}, Arc, Mutex};

use lil_broker::{Database, Timestamp};

use crate::uav::UAV;


pub struct RunnerOptions{
    pub dt: Timestamp,
    pub max_time: Timestamp,
}

pub enum RunnerCommand{
    Start
}
#[derive(Clone, Debug, PartialEq)]
pub enum RunnerStatus{
    Idle,
    Running,
    Finished
}

pub struct RunnerState{
    pub status: RunnerStatus,
    pub time: Timestamp,
}
pub struct RunnerChannels{
    pub database_arc: Arc<Mutex<Database>>,
    pub command_channel: (Sender<RunnerCommand>, Receiver<RunnerCommand>),
    pub state_channel: (Sender<RunnerState>, Receiver<RunnerState>),
}


impl RunnerChannels{
    pub fn new(uav: &UAV) -> Self{
        let (command_tx, command_rx) = std::sync::mpsc::channel();
        let (state_tx, state_rx) = std::sync::mpsc::channel();
        RunnerChannels{
            database_arc: uav.data.clone(),
            command_channel: (command_tx, command_rx),
            state_channel: (state_tx, state_rx),
        }
    }

}

pub struct Runner{
    pub uav: UAV,
    pub state: RunnerState,
    pub channels: RunnerChannels,
    pub options: RunnerOptions,
}


impl Runner{
    pub fn new(uav: UAV, opts: RunnerOptions) -> Self{
        let channels = RunnerChannels::new(&uav);
        Runner{
            uav,
            state: RunnerState{time: Timestamp::zero(), status: RunnerStatus::Idle},
            channels: channels,
            options: opts,
        }
    }

    pub fn start_thread(&mut self, options: RunnerOptions){
        self.state.status = RunnerStatus::Running;

        while self.state.status == RunnerStatus::Running{
            let runner_state = &self.state.status.clone();

            
        }
    }

    pub fn step(&mut self, runner_tate: &RunnerState){
        
    }
}
