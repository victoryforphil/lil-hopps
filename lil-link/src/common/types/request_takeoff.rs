use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadTakeoffRequest {
    pub height: f32,
    pub ack: bool,
}

impl QuadTakeoffRequest {
    pub fn new(height: f32) -> Self {
        Self { height, ack: false }
    }

    pub fn get_topic_key(&self) -> TopicKey {
        TopicKey::from_str("cmd/takeoff")
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }
}
