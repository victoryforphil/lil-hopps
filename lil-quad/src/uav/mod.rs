mod hardware;
mod quads;
mod tasks;
use std::sync::{Arc, Mutex};


use lil_broker::{Database, Timestamp};

pub use tasks::*;



pub struct UAV {
    tasker: TaskManager,
    pub data: Arc<Mutex<Database>>,
    runtime: Arc<Mutex<dyn UAVRuntime>>,
}

pub trait UAVRuntime {
    fn get_tasks(&self) -> Vec<TaskHandle>;
    fn get_active_tasks(&self) -> Vec<String>;
    fn inital_state(&mut self, db: &mut Database);
}

impl UAV {
    pub fn new(runtime_arc: Arc<Mutex<dyn UAVRuntime>>) -> Self {
        let mut task_manager = TaskManager::new();
        let mut runtime = runtime_arc.lock().unwrap();
        let mut data = Database::new();
        for task in runtime.get_tasks() {
            task_manager.add_task(task);
        }
        runtime.inital_state(&mut data);
        UAV {
            tasker: task_manager,
            runtime: runtime_arc.clone(),
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn tick(&mut self, timestamp: &Timestamp) -> Result<(), anyhow::Error> {
        let active_task = self.runtime.lock().unwrap().get_active_tasks();
        self.tasker.active_tasks = active_task;
        self.tasker.tick(timestamp, self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lil_broker::Primatives;
    use pretty_assertions::assert_eq;
    use tests::quads::TestQuadRuntime;
    use tracing::debug;
    #[test]
    fn test_uav() {
        env_logger::init();

        let runtime = TestQuadRuntime {};

        let mut uav = UAV::new(Arc::new(Mutex::new(runtime)));
        let t = Timestamp::new(0);
        uav.tick(&t).unwrap();

        // Check /math/output/echo for the result
        let dp = uav
            .data.lock().unwrap()
            .query_get_latest(vec!["/math/output/echo".to_string()].into())
            .unwrap();
        debug!("{:?}", dp);
        let echo = dp.data.get("/math/output/echo").unwrap();
        assert_eq!(echo[0].data, Primatives::Number(30.0));
    }
}
