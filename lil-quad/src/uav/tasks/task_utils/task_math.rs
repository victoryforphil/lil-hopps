use lil_broker::QueryResponse;

use crate::uav::{Task, TaskMetadata, TaskResult, TaskSubscription};

/// MathTask - Performs basic operations based on given inputs
/// Inputs:
/// - `/math/0`: [DataPoint::Number]
/// - `/math/1`: [DataPoint::Number]
/// - `/math/operation`: [DataPoint::String]
/// Outputs:
/// - `/math/output`: [DataPoint::Number]
pub struct MathTask {
    pub topic_a: TaskSubscription,
    pub topic_b: TaskSubscription,
}

impl MathTask {
    pub fn new(topic_a: TaskSubscription, topic_b: TaskSubscription) -> MathTask {
        MathTask { topic_a, topic_b }
    }
}

impl Task for MathTask {
    fn metadata(&self) -> TaskMetadata {
        TaskMetadata::new("MathTask".to_string())
            .with_subscriptions(vec![
                self.topic_a.clone(),
                self.topic_b.clone(),
                TaskSubscription::from("/math/operation".to_string()),
            ])
            .with_refresh_rate_hz(10.0)
    }
    //#[instrument(skip_all)]
    fn run(
        &mut self,
        t: &lil_broker::Timestamp,
        inputs: &std::collections::BTreeMap<String, QueryResponse>,
    ) -> Result<TaskResult, anyhow::Error> {
        let mut data = std::collections::BTreeMap::new();
        let topic0 = inputs
            .get(&self.topic_a.name)
            .unwrap()
            .to_json(&self.topic_a.name);
        let topic1 = inputs
            .get(&self.topic_b.name)
            .unwrap()
            .to_json(&self.topic_b.name);
        let operation = inputs
            .get("/math/operation")
            .unwrap()
            .to_json("/math/operation");

        let topic0_value = topic0.as_f64().unwrap();
        let topic1_value = match topic1.as_f64() {
            Some(value) => value,
            None => {
                return Err(anyhow::anyhow!(
                    "Invalid value for topic 1,got {:?}",
                    topic1
                ))
            }
        };
        let operation_value = operation.as_str().unwrap();
        let result = match operation_value {
            "+" => topic0_value + topic1_value,
            "-" => topic0_value - topic1_value,
            "*" => topic0_value * topic1_value,
            "/" => topic0_value / topic1_value,
            _ => return Err(anyhow::anyhow!("Invalid operation: {}", operation_value)),
        };
        let result_dp =
            lil_broker::DataPoint::new(t.clone(), lil_broker::Primatives::Number(result));
        data.insert("/math/output".to_string(), result_dp);
        Ok(TaskResult {
            data,
            execution_time: t.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lil_broker::Primatives;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use tracing::info;
    #[test]
    fn test_math_task_metadata() {
        let task = MathTask::new("/math/0".into(), "/math/1".into());
        let metadata = task.metadata();
        assert_eq!(metadata.name, "MathTask");
        assert_eq!(metadata.subscriptions.len(), 3);
        assert_eq!(metadata.subscriptions[0].name, "/math/0".to_string());
        assert_eq!(metadata.subscriptions[1].name, "/math/1".to_string());
        assert_eq!(
            metadata.subscriptions[2].name,
            "/math/operation".to_string()
        );
        assert_eq!(metadata.refresh_rate, lil_broker::Timestamp::from_hz(10.0));
    }

    #[test]
    fn test_math_task_run() {
        //env_logger::init();
        let mut task = MathTask::new("/math/0".into(), "/math/1".into());
        let t = lil_broker::Timestamp::new(0);
        let mut inputs = std::collections::BTreeMap::new();
        inputs.insert(
            "/math/0".into(),
            lil_broker::QueryResponse::from_json(json!({"/math/0":{"0": 3.0}})),
        );
        inputs.insert(
            "/math/1".into(),
            lil_broker::QueryResponse::from_json(json!({"/math/1":{"0": 5.0}})),
        );
        inputs.insert(
            "/math/operation".into(),
            lil_broker::QueryResponse::from_json(json!({"/math/operation":{"0": "+"}})),
        );
        info!("Inputs: {:#?}", inputs);
        let result = task.run(&t, &inputs).unwrap();
        assert_eq!(result.data.len(), 1);
        assert_eq!(
            result.data.get("/math/output").unwrap().data,
            Primatives::Number(8.0)
        );
    }
}
