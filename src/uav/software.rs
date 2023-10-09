use super::state::UAVState;

pub struct UAVSoftware{
     
}

impl UAVSoftware{
    pub fn new() -> Self{
        UAVSoftware{
            
        }
    }

    pub fn process(&mut self, t: f64, dt: f32, in_state: &UAVState) -> Result<UAVState, String>{
        println!("UAVSoftware process");
        
        Ok(UAVState::new(in_state.pose))
    }
}