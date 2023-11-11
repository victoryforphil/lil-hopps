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

    /// TEST: test_visualization
    /// ----
    /// Expectation(s):
    /// - Visualization::new() returns a Visualization
    /// - Visualization::init() returns ()
    /// - Visualization::render() returns ()
    /// 
    /// Failure(s):
    /// - Any runtime expections / panicks.
    #[test]
    fn test_visualization(){
        let mut vis = Visualization::new();
        vis.init();
        vis.render(0.01);
    }
}