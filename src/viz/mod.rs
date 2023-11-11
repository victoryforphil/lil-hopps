pub struct Visualization{
    
}

impl Visualization{
    pub fn new() -> Self{
        Visualization{
            
        }
    }

    pub fn init(&mut self){
        println!("Visualization init");
    }

    pub fn render(&mut self, _dt: f32){
        println!("Visualization step");
    }
}