use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArduMode {
    Stabilize = 0,
    Acro = 1,
    AltHold = 2,
    Auto = 3,
    Guided = 4,
    Loiter = 5,
    RTL = 6,
    Circle = 7,
    Land = 9,
    Drift = 11,
    Sport = 13,
    Flip = 14,
    AutoTune = 15,
    PosHold = 16,
    Brake = 17,
    Throw = 18,
    AvoidADSB = 19,
    GuidedNoGPS = 20,
    SmartRTL = 21,
    FlowHold = 22,
    Follow = 23,
    ZigZag = 24,
    SystemID = 25,
    HeliAutorotate = 26,
    AutoRTL = 27,
    Turtle = 28,
}

impl ArduMode {
    pub fn from_u32(value: u32) -> Option<ArduMode> {
        match value {
            0 => Some(ArduMode::Stabilize),
            1 => Some(ArduMode::Acro),
            2 => Some(ArduMode::AltHold),
            3 => Some(ArduMode::Auto),
            4 => Some(ArduMode::Guided),
            5 => Some(ArduMode::Loiter),
            6 => Some(ArduMode::RTL),
            7 => Some(ArduMode::Circle),
            9 => Some(ArduMode::Land),
            11 => Some(ArduMode::Drift),
            13 => Some(ArduMode::Sport),
            14 => Some(ArduMode::Flip),
            15 => Some(ArduMode::AutoTune),
            16 => Some(ArduMode::PosHold),
            17 => Some(ArduMode::Brake),
            18 => Some(ArduMode::Throw),
            19 => Some(ArduMode::AvoidADSB),
            20 => Some(ArduMode::GuidedNoGPS),
            21 => Some(ArduMode::SmartRTL),
            22 => Some(ArduMode::FlowHold),
            23 => Some(ArduMode::Follow),
            24 => Some(ArduMode::ZigZag),
            25 => Some(ArduMode::SystemID),
            26 => Some(ArduMode::HeliAutorotate),
            27 => Some(ArduMode::AutoRTL),
            28 => Some(ArduMode::Turtle),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn to_string(&self) -> String {
        match self {
            ArduMode::Stabilize => "Stabilize",
            ArduMode::Acro => "Acro",
            ArduMode::AltHold => "AltHold",
            ArduMode::Auto => "Auto",
            ArduMode::Guided => "Guided",
            ArduMode::Loiter => "Loiter", 
            ArduMode::RTL => "RTL",
            ArduMode::Circle => "Circle",
            ArduMode::Land => "Land",
            ArduMode::Drift => "Drift",
            ArduMode::Sport => "Sport",
            ArduMode::Flip => "Flip",
            ArduMode::AutoTune => "AutoTune",
            ArduMode::PosHold => "PosHold",
            ArduMode::Brake => "Brake",
            ArduMode::Throw => "Throw",
            ArduMode::AvoidADSB => "Avoid_ADSB",
            ArduMode::GuidedNoGPS => "Guided_NoGPS",
            ArduMode::SmartRTL => "Smart_RTL",
            ArduMode::FlowHold => "FlowHold",
            ArduMode::Follow => "Follow",
            ArduMode::ZigZag => "ZigZag",
            ArduMode::SystemID => "SystemID",
            ArduMode::HeliAutorotate => "Heli_Autorotate",
            ArduMode::AutoRTL => "Auto RTL",
            ArduMode::Turtle => "Turtle",
        }.to_string()
    }
}
