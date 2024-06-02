

use tracing::instrument;

use crate::uav::{Task, TaskMetadata, TaskResult, TaskSubscription};

pub struct HoverTask {
   
}

impl HoverTask {
    pub fn new() -> HoverTask {
        HoverTask {  }
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
        inputs: &std::collections::BTreeMap<String, QueryResponset>,
    ) -> Result<TaskResult, anyhow::Error> {
        let mut data = std::collections::BTreeMap::new();
        for (topic, dp) in inputs.iter() {
            data.insert(topic.clone() + "/echo", dp.clone());
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
        let inputs = {
            let mut map = std::collections::BTreeMap::new();
            map.insert(
                "/topic/0".into(),
                lil_broker::DataPoint::new(t.clone(), Primatives::Number(5.0)),
            );
            map.insert(
                "/topic/1".into(),
                lil_broker::DataPoint::new(t.clone(), Primatives::String("lil-hopps".to_string())),
            );
            map
        };
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
