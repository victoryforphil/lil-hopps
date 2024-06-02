

use lil_broker::{DataPoint, QueryResponse, Timestamp};
use lil_helper::types::Pose;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, instrument};

use crate::uav::{Task, TaskMetadata, TaskResult, TaskSubscription};
pub struct HoverTask {
   pub initial_pose: Pose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverOutputs{
    pub error: f32,
    pub desired_pose: Pose,
}
impl HoverTask {
    pub fn new() -> HoverTask {
        HoverTask { 
            initial_pose: Pose::default(),
         }
    }
}

impl Task for HoverTask {
    fn metadata(&self) -> TaskMetadata {
        TaskMetadata::new("HoverTask".to_string())
            .with_subscriptions(
                vec![
                    TaskSubscription::from("sense/pose".to_string()),
                ]
            )
            .with_refresh_rate_hz(50.0)
    }
   // #[instrument(skip_all)]
    fn run(
        &mut self,
        t: &lil_broker::Timestamp,
        inputs: &std::collections::BTreeMap<String, QueryResponse>,
    ) -> Result<TaskResult, anyhow::Error> {
        info!("Hover Task: Running");
       
        let pose = inputs.get("sense/pose").expect("Sense/pose").to_json("sense/pose/");
        let pose: Pose = serde_json::from_value(pose).unwrap();
        
        if t.tick_ms != 0 {
           self.initial_pose = pose;
        }

        let error = pose.position - self.initial_pose.position;
        let error = error.norm();
        let desired_pose = Pose::new(self.initial_pose.position, self.initial_pose.orientation);
        let outputs = HoverOutputs {
            error,
            desired_pose,
        };
        let result_dp = DataPoint::json_to_datapoints(t.clone(), json!(outputs));
        // Prefix the data with the task name
        let result_dp = result_dp.into_iter().map(|(k, v)| {
            (format!("hover/{}", k), v)
        }).collect();

        Ok(TaskResult {
            data: result_dp,
            execution_time: t.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::uav::TaskManager;

    use super::*;
    use lil_broker::{Database, Primatives, WriteQuery};
    use pretty_assertions::assert_eq;
    use tracing::{info, warn};
    #[test]
    fn test_hover_task_metadata() {
       
        let task = HoverTask::new();
        let metadata = task.metadata();
        assert_eq!(metadata.name, "HoverTask");
        assert_eq!(metadata.subscriptions.len(), 1);
        assert_eq!(metadata.subscriptions[0].name, "sense/pose".to_string());

        assert_eq!(metadata.refresh_rate, lil_broker::Timestamp::from_hz(50.0));
    }

    #[test]
    fn test_hover_task_run() {  
        env_logger::init();
      
        let mut task_manager = TaskManager::new();
        let task = Arc::new(Mutex::new(HoverTask::new()));
        task_manager.add_task(task);
        task_manager.activate_all_tasks();


        let db_arc = Arc::new(Mutex::new(Database::new()));
        let pose = Pose::default();
        let queries = WriteQuery::from_json_batch(json!(pose), Timestamp::zero(), "sense/pose".to_string());
        {
            let mut db = db_arc.lock().unwrap();
            db.query_batch(queries).expect("Failed to write queries");
        }

        let timestamp = Timestamp::from_seconds(0.0);

        let max_t = Timestamp::from_seconds(5.0);
        let mut t = timestamp.clone();
        let mut got_data = false;
        while t < max_t {
            let arc = db_arc.clone();
            task_manager.tick(&t, arc).unwrap();
            t = t + Timestamp::from_seconds(0.1);
        }
        {
            let mut db = db_arc.lock().unwrap();
            let result_out = db
                .query_get_latest(vec!["hover".to_string()].into())
                .unwrap();

            let result_out =  result_out.to_json("hover/");
            match serde_json::from_value::<HoverOutputs>(result_out){
                Ok(value) => {
                    assert_eq!(value.error, 0.0);
                    got_data = true;
                }
                Err(e) => {
                   warn!("Error: {:#?}", e);
                }
            }
         

        }

        assert_eq!(got_data, true);

    
    }
}
