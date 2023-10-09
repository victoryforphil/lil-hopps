pub struct UAVSoftware{
     
}

impl UAVSoftware{
    pub fn new() -> Self{
        UAVSoftware{
            
        }
    }

    pub fn process(&mut self, t: f64, dt: f32, in_state: &Uav){
        println!("UAVSoftware process");
    }
}