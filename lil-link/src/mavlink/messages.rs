use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageRx{
    ParamValue(String, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuadMessageTx{
    ParamSet(String, f64),
}
