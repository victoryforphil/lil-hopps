use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    vec,
};

use lil_broker::{Primatives, Timestamp, WriteQuery};
use serde_json::{json, Value};

use crate::uav::{EchoTask, MathTask, TaskHandle, TaskSubscription, UAVRuntime};

pub struct FixtureQuadRuntime {
    pub tasks: Vec<TaskHandle>,
    pub init_state: BTreeMap<String, Value>,
}
impl FixtureQuadRuntime {
    pub fn new(tasks: Vec<TaskHandle>, init_state: BTreeMap<String, Value>) -> FixtureQuadRuntime {
        FixtureQuadRuntime { tasks, init_state }
    }

    pub fn new_blank() -> FixtureQuadRuntime {
        FixtureQuadRuntime {
            tasks: vec![],
            init_state: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, task: TaskHandle) {
        self.tasks.push(task);
    }

    pub fn with_task(mut self, task: TaskHandle) -> FixtureQuadRuntime {
        self.tasks.push(task);
        self
    }

    pub fn with_init_value(mut self, key: String, value: Value) -> FixtureQuadRuntime {
        self.init_state.insert(key, value);
        self
    }

    pub fn add_init_value(&mut self, key: String, value: Value) {
        self.init_state.insert(key, value);
    }
}

impl UAVRuntime for FixtureQuadRuntime {
    fn get_tasks(&self) -> Vec<TaskHandle> {
        self.tasks.clone()
    }

    fn inital_state(&mut self, db: &mut lil_broker::Database) {
        let mut write_queries = vec![];
        for (key, value) in self.init_state.iter() {
            write_queries.append(&mut WriteQuery::from_json_batch(
                value.clone(),
                Timestamp::zero(),
                key.clone(),
            ));
        }

        db.query_batch(write_queries).unwrap();
    }

    fn get_active_tasks(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|task| task.lock().unwrap().metadata().name.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_test_quad_runtime() {}
}
