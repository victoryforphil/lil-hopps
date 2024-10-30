use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadTakeoffRequest {
    pub height: f32,
    pub ack: bool,
}
