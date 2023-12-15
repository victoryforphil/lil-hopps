use crate::uav::state::UAVState;

use super::PIDController;

pub struct PositionControl{
    pub pid_x: PIDController,
    pub pid_y: PIDController,
    pub target_pos : Vec<f64>,
}

impl PositionControl{
    pub fn new() -> Self{
        PositionControl{
            pid_x: PIDController::new_clamped(0.7, 0.5, 0.01, -5.0, 5.0),
            pid_y: PIDController::new_clamped(0.7, 0.5, 0.01, -5.0, 5.0),
            target_pos: vec![0.0, 0.0]
        }
    }

    pub fn update(&mut self, state: &UAVState, dt: f64) -> [f64; 2]{
        let pos = state.pose.position;
        let x = self.pid_x.update(self.target_pos[0], pos[0] as f64, dt);
        let y = self.pid_y.update(self.target_pos[1], pos[1] as f64, dt);
        
        [x, y]
    }
}