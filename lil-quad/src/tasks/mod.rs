use std::{any, collections::BTreeMap};

use lil_broker::{DataPoint, Timestamp, WriteQuery};

pub struct TaskMetadata {
    pub name: String,
    pub subscriptions: Vec<String>,
    pub refresh_rate: Timestamp,
}

impl TaskMetadata {
    pub fn new(name: String) -> TaskMetadata {
        TaskMetadata {
            name,
            ..Default::default()
        }
    }

    pub fn with_subscription(mut self, subscription: String) -> TaskMetadata {
        self.subscriptions.push(subscription);
        self
    }

    pub fn with_subscriptions(mut self, subscriptions: Vec<String>) -> TaskMetadata {
        self.subscriptions.extend(subscriptions);
        self
    }

    pub fn with_refresh_rate(mut self, refresh_rate: Timestamp) -> TaskMetadata {
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn with_refresh_rate_hz(mut self, hz: f32) -> TaskMetadata {
        self.refresh_rate = Timestamp::from_hz(hz);
        self
    }
}

impl Default for TaskMetadata {
    fn default() -> Self {
        TaskMetadata {
            name: "Default Task".to_string(),
            subscriptions: Vec::new(),
            refresh_rate: Timestamp::from_hz(100.0 as f32), // 100 Hz
        }
    }
}

pub struct TaskResult {
    pub data: BTreeMap<String, Vec<DataPoint>>,
    pub execution_time: Timestamp,
}

pub trait Task {
    fn metadata(&self) -> TaskMetadata;
    fn run(
        &mut self,
        t: &Timestamp,
        inputs: BTreeMap<String, DataPoint>,
    ) -> Result<TaskResult, anyhow::Error>;
}

// --- Mock Section --- //

/// Mock implementation of the Task trait
/// Inputs:
/// - inputs: A map of topic names to DataPoints
///    - `/topic/0`: DataPoint::Number
/// Outputs:
/// - data: Output data for writing by the task
///     - `/debug/0`: [DataPoint::String]
pub struct MockTask {}

impl Task for MockTask {
    fn metadata(&self) -> TaskMetadata {
        TaskMetadata::new("Mock Task".into())
            .with_refresh_rate(Timestamp::from_hz(100.0 as f32))
            .with_subscription("/topic/0".into())
    }

    fn run(
        &mut self,
        t: &Timestamp,
        inputs: BTreeMap<String, DataPoint>,
    ) -> Result<TaskResult, anyhow::Error> {
        let mut result = TaskResult {
            data: BTreeMap::new(),
            execution_time: Timestamp::zero(),
        };
        let topic0 = inputs.get("/topic/0").unwrap();
        let topic0_data = topic0.data.clone();
        let topic0_value = match topic0_data {
            lil_broker::Primatives::Number(n) => n,
            _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", topic0_data)),
        };

        let debug_message = format!("topic_0={}", topic0_value);
        let debug_message_dp =
            DataPoint::new(t.clone(), lil_broker::Primatives::String(debug_message));
        result
            .data
            .insert("/debug/0".into(), vec![debug_message_dp]);
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_mock_task_metadata() {
        let metadata = MockTask {}.metadata();
        assert_eq!(metadata.name, "Mock Task");
        assert_eq!(metadata.subscriptions.len(), 1);
        assert_eq!(metadata.subscriptions[0], "/topic/0");
        assert_eq!(metadata.refresh_rate, Timestamp::from_hz(100.0 as f32));
    }

    #[test]
    fn test_mock_task_run() {
        let mut task = MockTask {};
        let t = Timestamp::new(0);
        let inputs = {
            let mut map = BTreeMap::new();
            map.insert("/topic/0".into(), DataPoint::new(t, 5.0.into()));
            map
        };
        let result = task.run(&t, inputs).unwrap();

        assert_eq!(result.data.len(), 1);
        assert_eq!(result.execution_time, Timestamp::zero());

        let debug_messages = result.data.get("/debug/0").unwrap();
        assert_eq!(debug_messages.len(), 1);
        let debug_message = &debug_messages[0];
        assert_eq!(debug_message.data, lil_broker::Primatives::String("topic_0=5".to_string()));
    }
}