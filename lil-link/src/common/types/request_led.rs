use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadLedRequest {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub ack: bool,
}

impl QuadLedRequest {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { 
            red,
            green, 
            blue,
            ack: false 
        }
    }

    pub fn get_topic_key() -> TopicKey {
        TopicKey::from_str("cmd/led")
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }
}
