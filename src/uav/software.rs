use super::state::UAVState;

pub struct UAVSoftware{
     
}

impl UAVSoftware{
    pub fn new() -> Self{
        UAVSoftware{
            
        }
    }

    pub fn process(&mut self, _t: f64, _dt: f32, in_state: &UAVState) -> Result<UAVState, String>{
        println!("UAVSoftware process");
        
        Ok(in_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{uav::state::UAVState, types::pose::Pose};

    #[test]
    fn test_uav_software(){
        let mut software = UAVSoftware::new();
        let state = UAVState::new(Pose::zero());
        let result = software.process(0.0, 0.0, &state);
        assert!(result.is_ok());
    }
}