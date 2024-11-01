use serde::{Deserialize, Serialize};
use victory_data_store::topics::TopicKey;

use crate::common::identifiers::{IDENT_BASE_STATUS, IDENT_STATUS_HEALTH};

use super::mode::QuadMode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadHealthStatus {
    pub healthy: bool,
    pub reason: Option<String>,
}

impl QuadHealthStatus {
    pub fn new(healthy: bool, reason: Option<String>) -> Self {
        Self { healthy, reason }
    }
}

impl QuadHealthStatus {
    pub fn get_topic_key() -> TopicKey {
        TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_HEALTH))
    }
}
