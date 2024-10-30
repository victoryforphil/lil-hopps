use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadArmRequest {
    pub arm: bool,
    pub ack: bool,
}

impl QuadArmRequest {
    pub fn new(arm: bool) -> Self {
        Self { arm, ack: false }
    }

    pub fn get_topic_key(&self) -> TopicKey {
        TopicKey::from_str("cmd/arm")
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }
}
