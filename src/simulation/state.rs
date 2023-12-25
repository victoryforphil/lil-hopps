use std::collections::HashMap;

use crate::logging::LogEntry;

use polars::frame::DataFrame;

use super::actors::{uav_actor::UAVActorResult, world_actor::WorldActorResult};
#[derive(Debug, Clone)]
pub struct SimulationState {
    pub uav_state: UAVActorResult,
    pub world_state: WorldActorResult,
    pub logs: HashMap<String, Vec<LogEntry>>,
    pub running: bool,
    pub time: f64,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            uav_state: UAVActorResult::new(),
            world_state: WorldActorResult::new(),
            logs: HashMap::new(),
            running: false,
            time: 0.0,
        }
    }

    pub fn get_df(&self, lable: String) -> DataFrame {
        let mut df = self.uav_state.get_df(lable.clone());
      
        df
    }

    pub fn clone_without_logs(&self) -> Self {
        SimulationState {
            uav_state: self.uav_state.clone(),
            world_state: self.world_state.clone(),
            logs: HashMap::new(),
            running: self.running,
            time: self.time,
        }
    }
  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = SimulationState::new();
        assert_eq!(state.running, false);
    }
}
