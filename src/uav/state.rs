use crate::types::{movement::Movement, pose::Pose};
#[derive(Debug, PartialEq, Clone)]
pub struct UAVState {
    pub pose: Pose,
    pub movenment: Movement,
    pub motors: [f32; 4],
}

impl UAVState {
    ///
    /// Creates a new UAVState with the given pose and zero movement and motors.
    ///
    pub fn new() -> Self {
        UAVState {
            pose: Pose::zero(),
            movenment: Movement::zero(),
            motors: [0.0; 4],
        }
    }

    ///
    /// Creates a new UAVState with the given pose and zero movement and motors.
    ///
    pub fn new_with_pose(init_pose: Pose) -> Self {
        UAVState {
            pose: init_pose,
            movenment: Movement::zero(),
            motors: [0.0; 4],
        }
    }

    // This function is used to set the motors of the UAV.
    // The motors are clamped to the range [0, 1].
    pub fn safe_set_motors(&mut self, motors: [f32; 4]) {
        for i in 0..4 {
            self.motors[i] = motors[i].max(0.0).min(1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = UAVState::new();
        assert_eq!(state.pose, Pose::zero());
        assert_eq!(state.movenment, Movement::zero());
        assert_eq!(state.motors, [0.0; 4]);
    }

    #[test]
    fn test_new_pose() {
        let pose = Pose::zero();
        let state = UAVState::new_with_pose(pose);
        assert_eq!(state.pose, pose);
        assert_eq!(state.movenment, Movement::zero());
        assert_eq!(state.motors, [0.0; 4]);
    }

    #[test]
    fn test_safe_set_motors() {
        let mut state = UAVState::new();
        state.safe_set_motors([0.0, 0.5, 1.0, 1.5]);
        assert_eq!(state.motors, [0.0, 0.5, 1.0, 1.0]);
    }
}
