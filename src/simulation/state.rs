use crate::{uav::state::UAVState, types::pose::Pose};

use super::actors::{uav_actor::UAVActorResult, world_actor::WorldActorResult};

pub struct SimulationState{
    pub uav_state: UAVActorResult,
    pub world_state: WorldActorResult,
}

impl SimulationState{
    pub fn new() -> Self{
        SimulationState{
            uav_state: UAVActorResult::new(UAVState::new(Pose::zero())),
            world_state: WorldActorResult::new(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let state = SimulationState::new();
        
    }
}