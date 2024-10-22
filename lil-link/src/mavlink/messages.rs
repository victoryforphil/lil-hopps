use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageRx{
    ParamValue(String, f64),
    Attitude(f64, f64, f64), // roll, pitch, yaw
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageTx{
    ParamSet(String, f64),
}
