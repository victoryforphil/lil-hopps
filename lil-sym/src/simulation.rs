use std::collections::HashMap;

use crate::{Scenario, SimulationContext, SimulationState, UAVActor, WorldActor};

pub struct Simulation{
    pub world: WorldActor,
    pub uavs: HashMap<u32, UAVActor>,
    pub context: SimulationContext,
    pub state: SimulationState,
}

impl Simulation{
    pub fn new(scenario: &dyn Scenario){
        let context = SimulationContext::new();
        let uavs = scenario.generate_uavs();
        
    }
}