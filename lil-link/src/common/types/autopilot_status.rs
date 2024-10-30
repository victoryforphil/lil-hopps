use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadAutopilotStatus {
    pub custom_mode_enabled: bool,
    pub test_enabled: bool,
    pub auto_enabled: bool,
    pub guided_enabled: bool,
    pub stabilize_enabled: bool,
    pub hil_enabled: bool,
    pub manual_input_enabled: bool,
    pub safety_armed: bool,
}
