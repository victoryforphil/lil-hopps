pub mod state;
pub mod mission;
pub use mission::UAVMission;

use crate::hardware::{self, navigation_hw::NavigationHardware, control_hw::ControlHardware};

pub struct UAV{
    pub navigation_hardware: Box<dyn NavigationHardware>,
    pub control_hardware: Box<dyn ControlHardware>,
    pub state: state::UAVState

}