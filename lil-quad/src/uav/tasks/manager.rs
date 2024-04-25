use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use lil_broker::{DataPoint, Database, QueryCommand, Timestamp, WriteQuery};
use tracing::{debug, error, info};

use crate::uav::TaskSubscription;

use super::{Task, TaskMetadata};
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Waiting,
    Skipping,
    Running,
}

pub struct TaskState {
    pub name: String,
    pub last_run: Timestamp,
    pub status: TaskStatus,
}

impl TaskState {
    pub fn new(name: String) -> TaskState {
        TaskState {
            name,
            ..Default::default()
        }
    }
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState {
            name: "Default Task".to_string(),
            last_run: Timestamp::from_seconds(0.0),
            status: TaskStatus::Waiting,
        }
    }
}

pub type TaskHandle = Arc<Mutex<dyn Task>>;

pub struct TaskEntry {
    pub name: String,
    pub task: TaskHandle,
    pub metadata: TaskMetadata,
    pub state: TaskState,
}

impl TaskEntry {
    pub fn new(task: TaskHandle) -> Self {
        let task_lock = task.lock().unwrap();
        let metadata = task_lock.metadata();
        TaskEntry {
            name: metadata.name.clone(),
            task: task.clone(),
            metadata: metadata.clone(),
            state: TaskState::new(metadata.name.clone()),
        }
    }
}
pub struct TaskManager {
    pub tasks: Vec<TaskEntry>, // State tracking
    pub active_tasks: Vec<String>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            active_tasks: Vec::new(),
        }
    }
    pub fn add_task(&mut self, task: TaskHandle) {
        let task = TaskEntry::new(task);
        info!("Adding task: {:?}", task.metadata.name);
        // Print subscriptions
        for sub in &task.metadata.subscriptions {
            info!("  Sub: {:?}", sub);
        }
        self.tasks.push(task);
    }
    pub fn activate_all_tasks(&mut self) {
        let tasks: Vec<String> = self.tasks.iter().map(|t| t.name.clone()).collect();
        self.active_tasks = tasks;
    }
    pub fn set_active_tasks(&mut self, tasks: Vec<String>) {
        self.active_tasks = tasks;
    }

    pub fn get_task_entry(&self, name: &str) -> Option<&TaskEntry> {
        self.tasks.iter().find(|t| t.name == name)
    }
    // Get the inner task Box
    pub fn get_task(&self, name: &str) -> Option<&TaskHandle> {
        self.get_task_entry(name).map(|t| &t.task)
    }

    // Get the innter task box as mut  ref to the Task
    pub fn get_task_mut(&mut self, name: &str) -> Option<&mut TaskHandle> {
        self.tasks
            .iter_mut()
            .find(|t| t.name == name)
            .map(|t| &mut t.task)
    }

    pub fn tick(
        &mut self,
        timestamp: &Timestamp,
        database: Arc<Mutex<Database>>,
    ) -> Result<(), anyhow::Error> {
        info!("TaskManager tick: {:?}", timestamp);
        let mut db = database.lock().unwrap();
        for task in self.tasks.iter_mut() {
            // Get the metadat for the task
            if !self.active_tasks.contains(&task.metadata.name) {
                continue;
            }
            let metadata = &task.metadata;
            let state = &mut task.state;
            if timestamp.tick_ms > 0 {
                if timestamp.tick_ms - state.last_run.tick_ms < metadata.refresh_rate.tick_ms {
                    state.status = TaskStatus::Skipping;
                    continue;
                }
            }
            debug!("Running task: {}", metadata.name);

            state.last_run = timestamp.clone();
            state.status = TaskStatus::Running;

            let subscriptions = &metadata.subscriptions;
            let query = TaskSubscription::generate_latest_query(subscriptions);

            let data = db.query_get_latest(query);

            let data = match data {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to get data for task: {} {:?}", metadata.name, err);
                    continue;
                }
            };
            // Select last
            let inputs: BTreeMap<String, DataPoint> = data
                .data
                .into_iter()
                .map(|(k, v)| {
                    let last = v.last().unwrap();
                    (k, last.clone())
                })
                .collect();

            let mut task_lock: std::sync::MutexGuard<'_, dyn Task> = task.task.lock().unwrap();
            let result = task_lock.run(timestamp, &inputs);

            match result {
                Ok(result) => {
                    let write_data = result.data;
                    let mut write_queries: Vec<QueryCommand> = vec![];

                    for (topic, dp) in write_data {
                        let query = WriteQuery::new(topic, dp.data, timestamp.clone());
                        write_queries.push(query.into());
                    }

                    debug!(
                        "Task {} completed, wrote {} data points",
                        metadata.name,
                        write_queries.len()
                    );
                    let _write_result = db.query_batch(write_queries);
                }
                Err(err) => {
                    error!("Task {} failed: {:?} ", metadata.name, err);
                }
            }
        }

        debug!("Task Database after tick: {:#?}", db.buckets.keys());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lil_broker::{Primatives, WriteQuery};

    use crate::uav::TaskResult;

    use super::*;

    pub struct TaskA {}
    pub struct TaskB {}

    impl Task for TaskA {
        fn metadata(&self) -> TaskMetadata {
            TaskMetadata::new("task_a".to_string())
                .with_refresh_rate(Timestamp::from_hz(1.0))
                .with_subscription("a/input".into())
        }

        fn run(
            &mut self,
            t: &Timestamp,
            inputs: &BTreeMap<String, DataPoint>,
        ) -> Result<TaskResult, anyhow::Error> {
            let mut result = TaskResult {
                data: BTreeMap::new(),
                execution_time: Timestamp::zero(),
            };
            let topic0 = inputs.get("a/input").unwrap();
            let topic0_data = topic0.data.clone();
            let topic0_value = match topic0_data {
                lil_broker::Primatives::Number(n) => n,
                _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", topic0_data)),
            };

            let new_value = topic0_value * 2.0;
            let new_dp = DataPoint::new(t.clone(), Primatives::Number(new_value));

            result.data.insert("a/output".into(), new_dp);
            Ok(result)
        }
    }

    impl Task for TaskB {
        fn metadata(&self) -> TaskMetadata {
            TaskMetadata::new("task_b".to_string())
                .with_refresh_rate(Timestamp::from_hz(1.0))
                .with_subscription("b/input".into())
        }

        fn run(
            &mut self,
            t: &Timestamp,
            inputs: &BTreeMap<String, DataPoint>,
        ) -> Result<TaskResult, anyhow::Error> {
            let mut result = TaskResult {
                data: BTreeMap::new(),
                execution_time: Timestamp::zero(),
            };
            let topic0 = inputs.get("b/input").unwrap();
            let topic0_data = topic0.data.clone();
            let topic0_value = match topic0_data {
                lil_broker::Primatives::Number(n) => n,
                _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", topic0_data)),
            };

            let new_value = topic0_value * 3.0;
            let new_dp = DataPoint::new(t.clone(), Primatives::Number(new_value));

            result.data.insert("b/output".into(), new_dp);
            Ok(result)
        }
    }

    #[test]
    fn test_task_manager_data_flow() {
        let mut task_manager = TaskManager::new();
        let task_a = Arc::new(Mutex::new(TaskA {}));
        let task_b = Arc::new(Mutex::new(TaskB {}));

        task_manager.add_task(task_a);
        task_manager.add_task(task_b);
        task_manager.activate_all_tasks();

        let db_arc = Arc::new(Mutex::new(Database::new()));

        let queries = vec![
            WriteQuery::new("a/input".into(), Primatives::Number(2.0), Timestamp::new(0)).into(),
            WriteQuery::new("b/input".into(), Primatives::Number(4.0), Timestamp::new(0)).into(),
        ];
        {
            let mut db = db_arc.lock().unwrap();
            db.query_batch(queries).expect("Failed to write queries");
        }

        let timestamp = Timestamp::from_seconds(0.0);

        let max_t = Timestamp::from_seconds(1.0);
        let mut t = timestamp.clone();

        while t < max_t {
            let arc = db_arc.clone();
            task_manager.tick(&t, arc).unwrap();
            t = t + Timestamp::from_seconds(1.0);
        }
        {
            let mut db = db_arc.lock().unwrap();
            let result_a = db
                .query_get_latest(vec!["a/output".to_string()].into())
                .unwrap();

            let a_out = result_a.data.get("a/output").unwrap();
            assert_eq!(a_out[0].data, Primatives::Number(4.0));

            let result_b = db
                .query_get_latest(vec!["b/output".to_string()].into())
                .unwrap();

            let b_out = result_b.data.get("b/output").unwrap();

            assert_eq!(b_out[0].data, Primatives::Number(12.0));
        }
    }
}
