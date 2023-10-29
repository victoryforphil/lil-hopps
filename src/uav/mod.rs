use crate::types::{pose::Pose, movement::Movement, motors::Motor};

use self::{software::UAVSoftware, state::UAVState, config::UAVConfig};

pub mod software;
pub mod state;
pub mod config;

pub struct UAV{
    pub state: UAVState,
    pub software: UAVSoftware,
    pub config: UAVConfig,
    pub motors: Vec<Motor>,
}


impl UAV{
    pub fn new() -> Self{
        let config = UAVConfig::new_250mm();
        let mut motors = Vec::new();

        for i in 0..4{
            motors.push(Motor::new(i as u8, config));
        }

        UAV{
            state: UAVState::new(Pose::zero()),
            software: UAVSoftware::new(),
            config: config,
            motors: motors,
        }
    }

    pub fn process(&mut self, t: f64, dt: f32) -> Result<(), String>{
        self.state = self.software.process(t, dt, &self.state)?;

        for i in 0..4{
            self.motors[i].set_input_scalar(self.state.motors[i]);
            let out = self.motors[i].get_physics();
            println!("Motor {} force: {:?}", i, out);
        }
        Ok(())
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