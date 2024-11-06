use std::{fmt::Display, str::FromStr};

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

/**
 * Helper Function
 */
impl FromStr for QuadMode {
    type Err = ();

    fn from_str(input: &str) -> Result<QuadMode, Self::Err> {
        match input {
            "Stabilize" => Ok(QuadMode::Stabilize),
            "Acro" => Ok(QuadMode::Acro),
            "AltHold" => Ok(QuadMode::AltHold),
            "Auto" => Ok(QuadMode::Auto),
            "Guided" => Ok(QuadMode::Guided),
            "Loiter" => Ok(QuadMode::Loiter),
            "Return" => Ok(QuadMode::Return),
            "Land" => Ok(QuadMode::Land),
            "PosHold" => Ok(QuadMode::PosHold),
            "Brake" => Ok(QuadMode::Brake),
            "Follow" => Ok(QuadMode::Follow),
            _ => Err(()),
        }
    }
}

