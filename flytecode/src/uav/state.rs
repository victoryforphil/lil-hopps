use wingman::types::{pose::Pose, movement::Movement};

use super::UAVMission;

pub struct UAVState{
    pub id: u64,
    pub pose: Pose,
    pub movement: Movement,
    pub mission: UAVMission
}

