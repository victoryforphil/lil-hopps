use crate::types::{movement::Movement, pose::Pose};
#[derive(Debug, PartialEq, Clone)]
pub struct UAVState{
    pub pose: Pose,
    pub movenment: Movement,
    pub motors: [f32; 4],
}

impl UAVState{
    pub fn new(init_pose: Pose) -> Self{
        UAVState{
            pose: init_pose,
            movenment: Movement::zero(),
            motors: [0.0; 4],
        }
    }

    // This function is used to set the motors of the UAV.
    // The motors are clamped to the range [0, 1].
    pub fn safe_set_motors(&mut self, motors: [f32; 4]){
        for i in 0..4{
            self.motors[i] = motors[i].max(0.0).min(1.0);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let state = UAVState::new(Pose::zero());
        assert_eq!(state.pose, Pose::zero());
        assert_eq!(state.movenment, Movement::zero());
        assert_eq!(state.motors, [0.0; 4]);
    }

    #[test]
    fn test_safe_set_motors(){
        let mut state = UAVState::new(Pose::zero());
        state.safe_set_motors([0.0, 0.5, 1.0, 1.5]);
        assert_eq!(state.motors, [0.0, 0.5, 1.0, 1.0]);
    }
}