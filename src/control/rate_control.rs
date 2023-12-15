use crate::uav::state::UAVState;

use super::PIDController;

pub struct RateControl{
    pub pid_x: PIDController,
    pub pid_y: PIDController,
    pub pid_z: PIDController,
    pub target_rates : Vec<f64>,
}

impl RateControl{
    pub fn new() -> Self{
        RateControl{
            pid_x: PIDController::new_clamped(0.2, 0.02, 0.01, -0.5, 0.5),
            pid_y: PIDController::new_clamped(0.2, 0.02, 0.01, -0.5, 0.5),
            pid_z: PIDController::new_clamped(0.2, 0.02, 0.01, -0.5, 0.5),
            target_rates: vec![0.0, 0.0, 0.0]
        }
    }

    pub fn update(&mut self, state: &UAVState, dt: f64) -> [f64; 3]{
        let rates = state.movenment.ang_vel;
        let x = self.pid_x.update(self.target_rates[0], rates[0] as f64, dt);
        let y = self.pid_y.update(self.target_rates[1], rates[1] as f64, dt);
        let z = self.pid_z.update(self.target_rates[2], rates[2] as f64, dt);
        
        [x, y, z]
    }
}