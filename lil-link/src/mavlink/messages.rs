use serde::{Deserialize, Serialize};

use super::types::{QuadMode, QuadModeStatus, QuadNED};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageRx{
    ParamValue(String, f64),
    Attitude(f64, f64, f64), // roll, pitch, yaw
    Position(QuadNED),
    ModeStatus(QuadModeStatus),
    SimpleStatus(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageTx{
    ParamSet(String, f64),
    SetArm(bool),
    SetMode(QuadMode),
    TakeOff(f32),
}


