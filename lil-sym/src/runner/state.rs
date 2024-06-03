use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lil_broker::{Database, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimRunnerStatus {
    Init,
    Running,
    Completed,
    Error,
}
#[derive(Debug, Clone)]
pub struct SimRunnerState {
    pub t: Timestamp,
    pub state: SimRunnerStatus,
    pub uav_dbs: HashMap<u32, Arc<Mutex<Database>>>,
}

impl Default for SimRunnerState {
    fn default() -> Self {
        SimRunnerState {
            t: Timestamp::from_seconds(0.0),
            state: SimRunnerStatus::Init,
            uav_dbs: HashMap::new(),
        }
    }
}

impl SimRunnerState {
    pub fn new(t: Timestamp, state: SimRunnerStatus) -> Self {
        SimRunnerState {
            t,
            state,
            uav_dbs: HashMap::new(),
        }
    }
}
