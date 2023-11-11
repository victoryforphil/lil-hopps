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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization(){
        let mut vis = Visualization::new();
        vis.init();
        vis.render(0.01);
    }
}