use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadArmRequest {
    pub arm: bool,
    pub ack: bool,
}
