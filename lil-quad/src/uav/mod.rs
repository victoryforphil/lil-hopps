mod hardware;
mod quads;
mod tasks;
pub use hardware::*;
use lil_broker::{Database, Timestamp};
pub use quads::*;
pub use tasks::*;

use self::{act::ActHardware, sense::SenseHardware};

pub struct UAV {
    tasker: TaskManager,
}

pub trait UAVRuntime {
    fn get_tasks(&self) -> Vec<TaskHandle>;
    fn get_active_tasks(&self) -> Vec<String>;
    fn inital_state(&mut self, db: &mut Database);
}

impl UAV {
    pub fn new(runtime: &mut dyn UAVRuntime) -> Self {
        let mut task_manager = TaskManager::new();
        for task in runtime.get_tasks() {
            task_manager.add_task(task);
        }
        task_manager.activate_all_tasks();
        runtime.inital_state(&mut task_manager.data);
        UAV {
            tasker: task_manager,
        }
    }

    pub fn tick(&mut self, timestamp: &Timestamp) -> Result<(), anyhow::Error> {
        self.tasker.tick(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lil_broker::Primatives;
    use pretty_assertions::assert_eq;
    use tracing::debug;
    #[test]
    fn test_uav() {
        env_logger::init();

        let mut runtime = TestQuadRuntime {};
        let mut uav = UAV::new(&mut runtime);
        let t = Timestamp::new(0);
        uav.tick(&t).unwrap();

        // Check /math/output/echo for the result
        let dp = uav
            .tasker
            .data
            .query_get_latest(vec!["/math/output/echo".to_string()].into())
            .unwrap();
        debug!("{:?}", dp);
        let echo = dp.data.get("/math/output/echo").unwrap();
        assert_eq!(echo[0].data, Primatives::Number(30.0));
    }
}
