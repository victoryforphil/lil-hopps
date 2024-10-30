use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

use crate::common::identifiers::IDENT_BASE_PARAMS;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuadParameter {
    pub param: String,
    pub value: f64,
    pub ack: bool,
}

impl QuadParameter {
    pub fn new(param: String, value: f64) -> Self {
        Self {
            param,
            value,
            ack: false,
        }
    }

    pub fn ack(&mut self) {
        self.ack = true;
    }

    pub fn byte_id(&self) -> [u8; 16] {
        self.param.as_bytes().try_into().unwrap()
    }

    pub fn get_topic_key(&self) -> TopicKey {
        TopicKey::from_str(&format!("{}/{}", IDENT_BASE_PARAMS, self.param))
    }
}
