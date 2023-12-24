mod loggable;
pub use loggable::Loggable;

use crate::types::{motors::Motor, movement::Movement, pose::Pose};
#[derive(Debug, PartialEq, Clone)]
pub struct LogEntry {
    pub time: f64,
    pub data: LogData,
    pub transmit: bool,
    pub key: String,
}
impl LogEntry {
    pub fn new(key: String, time: f64, data: LogData) -> Self {
        LogEntry {
            time,
            data,
            transmit: false,
            key,
        }
    }

    pub fn new_transmitable(key: String, time: f64, data: LogData) -> Self {
        LogEntry {
            time,
            data,
            transmit: true,
            key,
        }
    }

    pub fn prefix(&self, prefix: &str) -> Self {
        LogEntry {
            time: self.time,
            data: self.data.clone(),
            transmit: self.transmit,
            key: format!("{}{}", prefix, self.key),
        }
    }

    pub fn prefix_batch(entries: Vec<LogEntry>, prefix: &str) -> Vec<LogEntry> {
        entries
            .into_iter()
            .map(|entry| entry.prefix(prefix))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogData {
    Pose(Pose),
    Movement(Movement),
    Motor(Motor),
    String(String),
    Float(f32),
}
