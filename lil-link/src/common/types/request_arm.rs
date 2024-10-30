use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

use super::mode::QuadMode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadSetModeRequest {
    pub mode: QuadMode,
    pub ack: bool,
}

impl QuadSetModeRequest {
    pub fn new(mode: QuadMode) -> Self {
        Self { mode, ack: false }
    }
}

impl QuadSetModeRequest {
    pub fn get_topic_key(&self) -> TopicKey {
        TopicKey::from_str("cmd/mode")
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }
}
