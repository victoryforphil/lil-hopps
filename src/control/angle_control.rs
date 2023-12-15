use log::info;

use crate::uav::state::UAVState;

use super::PIDController;

pub struct AngleControl{
    pub pid_x: PIDController,
    pub pid_y: PIDController,
    pub pid_z: PIDController,
    pub target_angles : Vec<f64>,
}

impl AngleControl{
    pub fn new() -> Self{
        AngleControl{
            pid_x: PIDController::new_clamped(0.3, 0.01, 0.01, -1.0, 1.0),
            pid_y: PIDController::new_clamped(0.3, 0.01, 0.01, -1.0, 1.0),
            pid_z: PIDController::new_clamped(0.3, 0.01, 0.01, -1.0, 1.0),
            target_angles: vec![0.0, 0.0, 0.0]
        }
    }

    pub fn update(&mut self, state: &UAVState, dt: f64) -> [f64; 3]{
        let angles = state.pose.orientation.euler_angles();
        let x = self.pid_x.update(self.target_angles[0], angles.0 as f64, dt);
        let y = self.pid_y.update(self.target_angles[1], angles.1 as f64, dt);
        let z = self.pid_z.update(self.target_angles[2], angles.2 as f64, dt);
        
        [x, y, z]
    }
}