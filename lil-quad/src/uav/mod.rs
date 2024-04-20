mod software;
mod hardware;
pub use software::*;
pub use hardware::*;

use crate::tasks::Task;

use self::{act::ActHardware, sense::SenseHardware};


pub trait UAVRuntime{
    fn get_tasks(&self) -> Vec<Box<dyn Task>>;
    fn get_sense_hardware(&self) -> Box<dyn SenseHardware>;
    fn get_act_hardware(&self) -> Box<dyn ActHardware>;
    fn tick(&mut self) -> Result<(), anyhow::Error>;
}

pub struct UAV{
    tasks: Vec<Box<dyn Task>>,
    sense_hardware: Box<dyn SenseHardware>,
    act_hardware: Box<dyn ActHardware>,
    broker: lil_broker::Database,
}

