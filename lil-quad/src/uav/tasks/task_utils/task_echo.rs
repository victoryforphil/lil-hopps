use lil_broker::{DataPoint, QueryResponse};
use tracing::{debug, info, instrument};

use crate::uav::{Task, TaskMetadata, TaskResult, TaskSubscription};

pub struct EchoTask {
    pub echo_topics: Vec<String>,
}

impl EchoTask {
    pub fn new(echo_topics: Vec<String>) -> EchoTask {
        EchoTask { echo_topics }
    }
}

impl Task for EchoTask {
    fn metadata(&self) -> TaskMetadata {
        TaskMetadata::new("EchoTask".to_string())
            .with_subscriptions(
                self.echo_topics
                    .iter()
                    .map(|topic| TaskSubscription::from(topic.clone()))
                    .collect(),
            )
            .with_refresh_rate_hz(10.0)
    }
    // #[instrument(skip_all)]
    fn run(
        &mut self,
        t: &lil_broker::Timestamp,
        inputs: &std::collections::BTreeMap<String, QueryResponse>,
    ) -> Result<TaskResult, anyhow::Error> {
        let mut data = std::collections::BTreeMap::new();
        for (topic, response) in inputs.iter() {
            for (key, value) in &response.data {
                info!("{}: {:?}", key, value);
                data.insert(format!("{}/echo", topic), value.last().unwrap().clone());
            }
        }
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
    #[test]
    fn test_echo_task_metadata() {
        let echo_topics = vec!["/topic/0".to_string(), "/topic/1".to_string()];
        let task = EchoTask::new(echo_topics);
        let metadata = task.metadata();
        assert_eq!(metadata.name, "EchoTask");
        assert_eq!(metadata.subscriptions.len(), 2);
        assert_eq!(metadata.subscriptions[0].name, "/topic/0".to_string());
        assert_eq!(metadata.subscriptions[1].name, "/topic/1".to_string());
        assert_eq!(metadata.refresh_rate, lil_broker::Timestamp::from_hz(10.0));
    }

    #[test]
    fn test_echo_task_run() {
        let echo_topics = vec!["/topic/0".to_string(), "/topic/1".to_string()];
        let mut task = EchoTask::new(echo_topics);
        let t = lil_broker::Timestamp::new(0);
        let mut inputs = std::collections::BTreeMap::new();
        inputs.insert(
            "/topic/0".into(),
            lil_broker::QueryResponse::from_json(json!({"/topic/0": {"0": 5.0}})),
        );
        inputs.insert(
            "/topic/1".into(),
            lil_broker::QueryResponse::from_json(json!({"/topic/1": {"0": "lil-hopps"}})),
        );

        let result = task.run(&t, &inputs).unwrap();
        assert_eq!(result.data.len(), 2);
        assert_eq!(
            result.data.get("/topic/0/echo").unwrap().data,
            Primatives::Number(5.0)
        );
        assert_eq!(
            result.data.get("/topic/1/echo").unwrap().data,
            Primatives::String("lil-hopps".to_string())
        );
    }
}
