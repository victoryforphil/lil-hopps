use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuadMode {
    Stabilize,
    Acro,
    AltHold,
    Auto,
    Guided,
    Loiter,
    Return,
    Land,
    PosHold,
    Brake,
    Follow,
}

impl Display for QuadMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
