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

    fn run(
        &mut self,
        t: &lil_broker::Timestamp,
        inputs: &std::collections::BTreeMap<String, lil_broker::DataPoint>,
    ) -> Result<TaskResult, anyhow::Error> {
        let mut data = std::collections::BTreeMap::new();
        let topic0 = inputs.get(&self.topic_a.name).unwrap();
        let topic1 = inputs.get(&self.topic_b.name).unwrap();
        let operation = inputs.get("/math/operation").unwrap();
        let topic0_data = topic0.data.clone();
        let topic1_data = topic1.data.clone();
        let operation_data = operation.data.clone();
        let topic0_value = match topic0_data {
            lil_broker::Primatives::Number(n) => n,
            _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", topic0_data)),
        };
        let topic1_value = match topic1_data {
            lil_broker::Primatives::Number(n) => n,
            _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", topic1_data)),
        };
        let operation_value = match operation_data {
            lil_broker::Primatives::String(s) => s,
            _ => return Err(anyhow::anyhow!("Expected String, got {:?}", operation_data)),
        };
        let result = match operation_value.as_str() {
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
        let mut task = MathTask::new("/math/0".into(), "/math/1".into());
        let t = lil_broker::Timestamp::new(0);
        let inputs = {
            let mut map = std::collections::BTreeMap::new();
            map.insert(
                "/math/0".into(),
                lil_broker::DataPoint::new(t.clone(), Primatives::Number(5.0)),
            );
            map.insert(
                "/math/1".into(),
                lil_broker::DataPoint::new(t.clone(), Primatives::Number(3.0)),
            );
            map.insert(
                "/math/operation".into(),
                lil_broker::DataPoint::new(t.clone(), Primatives::String("+".to_string())),
            );
            map
        };
        let result = task.run(&t, &inputs).unwrap();
        assert_eq!(result.data.len(), 1);
        assert_eq!(
            result.data.get("/math/output").unwrap().data,
            Primatives::Number(8.0)
        );
    }
}
