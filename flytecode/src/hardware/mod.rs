pub mod control_hw;
pub mod navigation_hw;


// Base hardware trait

pub trait Hardware{
    fn init(&mut self) -> Result<(), String>;
    fn update(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn get_status(&self) -> Result<(), String>;
}