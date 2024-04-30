use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lil_broker::Timestamp;
use lil_quad::{
    runner::{UAVRunner, UAVRunnerConfig, UAVThreadedRunner},
    uav::{MockUAVRuntime, UAV},
};

use crate::UAVActor;

pub trait Scenario: Send + Sync {
    fn generate_uavs(&self) -> HashMap<u32, UAVActor>;
}

/// DefaultScenarios
/// ----
/// Generates a single UAV (id=0) at (0,0,0);
#[derive(Debug, Clone, Default)]
pub struct DefaultScenario {}
impl Scenario for DefaultScenario {
    fn generate_uavs(&self) -> HashMap<u32, UAVActor> {
        let mut uavs = HashMap::new();

        let uav_runtime = MockUAVRuntime::new();
        let uav_runtime = Arc::new(Mutex::new(uav_runtime));
        let uav = UAV::new(uav_runtime);
        let runner = UAVThreadedRunner::new(
            uav,
            UAVRunnerConfig {
                dt: Timestamp::from_hz(100.0),
                max_t: Timestamp::from_seconds(30.0),
                external_tick: true,
                wait: true,
            },
        );

        uavs.insert(0, UAVActor::new(runner));

        uavs
    }
}
