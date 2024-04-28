use std::collections::HashMap;

use crate::UAVActor;



pub trait Scenario: Send + Sync{
    fn generate_uavs(&self) -> HashMap<u32, UAVActor>;
}

/// DefaultScenarios
/// ----
/// Generates a single UAV (id=0) at (0,0,0);
#[derive(Debug, Clone, Default)]
pub struct DefaultScenario{}
impl Scenario for DefaultScenario{
    fn generate_uavs(&self) -> HashMap<u32, UAVActor>{
        let mut uavs = HashMap::new();
        //TODO: Fill this out [@victoryforphil]
        uavs
    }
}