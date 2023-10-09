use crate::types::{pose::Pose, movement::Movement};

use self::{software::UAVSoftware, state::UAVState};

pub mod software;
pub mod state;

pub struct UAV{
    pub state: UAVState,
    pub software: UAVSoftware
}


impl UAV{
    pub fn new() -> Self{
        UAV{
            state: UAVState::new(Pose::zero()),
            software: UAVSoftware::new(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let uav = UAV::new();
        assert_eq!(uav.state, UAVState::new(Pose::zero()));
        
    }
}