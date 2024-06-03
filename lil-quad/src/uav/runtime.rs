use std::sync::{Arc, Mutex};

use lil_broker::{Database, Timestamp};

use crate::uav::MockTask;

use super::TaskHandle;

pub trait UAVRuntime: Send + Sync {
    fn get_tasks(&self) -> Vec<TaskHandle>;
    fn get_active_tasks(&self) -> Vec<String>;
    fn inital_state(&mut self, db: &mut Database);

    fn as_arc_mutex(self) -> Arc<Mutex<Self>>
    where
        Self: Sized,
    {
        Arc::new(Mutex::new(self))
    }
}

// --- Mock Section --- //

/// MockUAVRuntime
/// ----
/// A mock implementation of the UAVRuntime trait that
/// implmenets MockTask which is always active.
pub struct MockUAVRuntime {}
impl MockUAVRuntime {
    pub fn new() -> Self {
        Self {}
    }
}
impl UAVRuntime for MockUAVRuntime {
    fn get_tasks(&self) -> Vec<TaskHandle> {
        let mock_task = MockTask {};
        let task_handle = Arc::new(Mutex::new(mock_task));
        vec![task_handle]
    }

    fn get_active_tasks(&self) -> Vec<String> {
        vec!["MockTask".to_string()]
    }

    fn inital_state(&mut self, db: &mut Database) {
        db.set_time(Timestamp::zero());
        db.quick_write("/topic/0", 7.0.into()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_runtime_mock_get_tasks() {
        let runtime = MockUAVRuntime::new();
        let tasks = runtime.get_tasks();
        assert_eq!(tasks.len(), 1);
        for task in tasks {
            assert_eq!(task.lock().unwrap().metadata().name, "MockTask");
        }
    }

    #[test]
    pub fn test_runtime_mock_get_active_tasks() {
        let runtime = MockUAVRuntime::new();
        let active_tasks = runtime.get_active_tasks();
        assert_eq!(active_tasks.len(), 1);
        assert_eq!(active_tasks[0], "MockTask");
    }

    #[test]
    pub fn test_runtime_mock_inital_state() {
        let mut db = Database::new();
        let mut runtime = MockUAVRuntime::new();
        runtime.inital_state(&mut db);
        let keys = db.get_keys();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "/topic/0");
    }
}
