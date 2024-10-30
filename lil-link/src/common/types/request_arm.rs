use serde::{Deserialize, Serialize};

use super::mode::QuadMode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadSetModeRequest {
    pub mode: QuadMode,
    pub ack: bool,
}
