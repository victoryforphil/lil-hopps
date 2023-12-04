use crate::types::{motors::Motor};

use self::{config::UAVConfig, software::UAVSoftware, state::UAVState};

pub mod config;
pub mod software;
pub mod state;

pub struct UAV {
    pub state: UAVState,
    pub software: UAVSoftware,
    pub config: UAVConfig,
    pub motors: Vec<Motor>,
}

impl UAV {
    pub fn new() -> Self {
        let config = UAVConfig::new_250mm();
        let mut motors = Vec::new();

        for i in 0..4 {
            motors.push(Motor::new(i as u8, config));
        }

        UAV {
            state: UAVState::new(),
            software: UAVSoftware::new(),
            config: config,
            motors: motors,
        }
    }

    pub fn process(&mut self, t: f64, dt: f32) -> Result<(), String> {
        self.state = self.software.process(t, dt, &self.state)?;

        for i in 0..4 {
            self.motors[i].set_input_scalar(self.state.motors[i]);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uav = UAV::new();
        assert_eq!(uav.state, UAVState::new());
        assert_eq!(uav.motors.len(), 4);
    }

    #[test]
    fn test_process() {
        let mut uav = UAV::new();
        uav.state.motors = [0.0, 0.5, 1.0, 1.5];
        let result = uav.process(0.0, 0.0);
        assert!(result.is_ok());
        assert_eq!(uav.motors[0].current_value, 0.0);
        assert_eq!(uav.motors[1].current_value, 0.5);
        assert_eq!(uav.motors[2].current_value, 1.0);
        assert_eq!(uav.motors[3].current_value, 1.5);
    }
}
