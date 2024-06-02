use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    vec,
};


use serde_json::{Value};

use crate::uav::{FixtureQuadRuntime, TaskHandle, UAV};

use super::{UAVRunner, UAVRunnerConfig};

pub struct FixtureRunner {}

impl FixtureRunner {
    pub fn new(
        config: UAVRunnerConfig,
        task: TaskHandle,
        state: BTreeMap<String, Value>,
    ) -> Result<UAVRunner, anyhow::Error> {
        let runtime = FixtureQuadRuntime::new(vec![task], state);

        let runtime = Arc::new(Mutex::new(runtime));
        let uav = UAV::new(runtime);
        let mut runner = UAVRunner::new(config.clone(), uav);
        runner.init()?;
        runner.send_start();
        Ok(runner)
    }
}
