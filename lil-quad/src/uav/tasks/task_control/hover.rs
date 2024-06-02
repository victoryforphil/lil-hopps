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
pub struct HoverOutputs {
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
            .with_subscriptions(vec![TaskSubscription::from("sense/pose".to_string())])
            .with_refresh_rate_hz(50.0)
    }
    // #[instrument(skip_all)]
    fn run(
        &mut self,
        t: &lil_broker::Timestamp,
        inputs: &std::collections::BTreeMap<String, QueryResponse>,
    ) -> Result<TaskResult, anyhow::Error> {
        let pose = inputs
            .get("sense/pose")
            .expect("Sense/pose missing")
            .to_json("sense/pose/");
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
        let result_dp = result_dp
            .into_iter()
            .map(|(k, v)| (format!("hover/{}", k), v))
            .collect();

        Ok(TaskResult {
            data: result_dp,
            execution_time: t.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    use crate::{
        runner::{FixtureRunner, UAVRunner, UAVRunnerConfig},
        uav::{quads::FixtureQuadRuntime, TaskManager, UAV},
    };

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

        let mut pose = Pose::default();
        pose.position.x = 1.0;
        pose.position.y = 1.0;
        pose.position.z = 1.0;

        let mut init_state = BTreeMap::new();
        init_state.insert("sense/pose".to_string(), json!(pose));

        let task = Arc::new(Mutex::new(HoverTask::new()));
        let config = UAVRunnerConfig::default().set_max_t(Timestamp::from_seconds(5.0));

        let mut runner =
            FixtureRunner::new(config.clone(), task, init_state).expect("Failed to create runner");
        let state = runner.start().expect("Failed to start runner");
        assert_eq!(state.t, config.max_t);

        let mut db = runner.channels.database_arc.lock().unwrap();
        let result_out = db
            .query_get_latest(vec!["hover".to_string()].into())
            .unwrap();

        let result_out = result_out.to_json("hover/");
        let value = serde_json::from_value::<HoverOutputs>(result_out).unwrap();
        assert_eq!(value.error, 0.0);
        assert_eq!(value.desired_pose, pose);
    }
}
