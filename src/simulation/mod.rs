
pub struct Simulation{}


impl Simulation{
    pub fn new() -> Self{
        Simulation{
            
        }
    }

    pub fn init(&mut self){
        println!("Simulation init");
    }

    pub fn step(&mut self, dt: f32){
        println!("Simulation step");
    }
}