use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadLandRequest {
    pub ack: bool,
}

impl QuadLandRequest {
    pub fn new() -> Self {
        Self {  ack: false }
    }

    pub fn get_topic_key(&self) -> TopicKey {
        TopicKey::from_str("cmd/land")
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }
}
