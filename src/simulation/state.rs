use crate::{types::pose::Pose, uav::state::UAVState};

use super::actors::{uav_actor::UAVActorResult, world_actor::WorldActorResult};
#[derive(Debug, Clone)]
pub struct SimulationState {
    pub uav_state: UAVActorResult,
    pub world_state: WorldActorResult,
    pub running: bool,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            uav_state: UAVActorResult::new(),
            world_state: WorldActorResult::new(),
            running: false,
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
