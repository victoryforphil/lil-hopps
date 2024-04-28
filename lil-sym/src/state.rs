use std::collections::HashMap;

use lil_broker::Timestamp;

use crate::{UAVActorState, WorldActorResult};
#[derive(Debug, Clone, Default)]
pub struct SimulationState{
    pub uavs: HashMap<u32, UAVActorState>,
    pub world: WorldActorResult,
    pub time: Timestamp,
    pub running: bool
}

impl SimulationState{
    pub fn new() -> SimulationState{
        SimulationState{
            uavs: HashMap::new(),
            world: WorldActorResult::new(),
            time: Timestamp::zero(),
            running: false
        }
    }
}
