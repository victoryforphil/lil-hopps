
use std::{any, collections::BTreeMap};

use lil_broker::{DataPoint, Timestamp, WriteQuery};

pub struct TaskMetadata{
    pub name: String,
    pub subscriptions: Vec<String>,
    pub refresh_rate: Timestamp
}

impl TaskMetadata{
    pub fn new(name: String) -> TaskMetadata{
        TaskMetadata{
            name,
            ..Default::default()
        }
    }

    pub fn with_subscription(mut self, subscription: String) -> TaskMetadata{
        self.subscriptions.push(subscription);
        self
    }

    pub fn with_subscriptions(mut self, subscriptions: Vec<String>) -> TaskMetadata{
        self.subscriptions.extend(subscriptions);
        self
    }

    pub fn with_refresh_rate(mut self, refresh_rate: Timestamp) -> TaskMetadata{
        self.refresh_rate = refresh_rate;
        self
    }

    pub fn with_refresh_rate_hz(mut self, hz: f32) -> TaskMetadata{
        self.refresh_rate = Timestamp::from_hz(hz);
        self
    }
}

impl Default for TaskMetadata{
    fn default() -> Self{
        TaskMetadata{
            name: "Default Task".to_string(),
            subscriptions: Vec::new(),
            refresh_rate: Timestamp::from_hz(100.0 as f32) // 100 Hz
        }
    }
}

pub struct TaskResult{
    pub data: BTreeMap<String, Vec<DataPoint>>,
    pub execution_time: Timestamp
}

pub trait Task{
    fn metadata(&self) -> TaskMetadata;
    fn run(&mut self, t: &Timestamp, inputs: BTreeMap<String, DataPoint>) -> Result<TaskResult, anyhow::Error>;
}