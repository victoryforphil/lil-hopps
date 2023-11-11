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
        
        Ok(UAVState::new(in_state.pose))
    }
}