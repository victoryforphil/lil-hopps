use crate::uav::state::UAVState;

use super::PIDController;

pub struct HeightControl {
    pub pid: PIDController,
    pub target_height: f64,
    pub hover_throttle: f64,
}

impl HeightControl {
    pub fn new(target_height: f64, hover: f64) -> Self {
        HeightControl {
            pid: PIDController::new_clamped(0.5, 0.1, 0.3, -1.0, 1.0),
            target_height,
            hover_throttle: hover,
        }
    }

    pub fn update(&mut self, state: &UAVState, dt: f64) -> f64 {
        let height = state.pose.position[2];
        let output = self.pid.update(self.target_height, height as f64, dt);
        let out = output + self.hover_throttle;
        
        //Clamp output
        if out > 1.0 {
            1.0
        } else if out < 0.0 {
            0.0
        } else {
            out
        }
        
    }
}