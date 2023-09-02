use crate::uav::{state::UAVState, UAVMission};

use super::tasks::UAVTask;


pub struct TaskGroup{
    pub name: String,
    pub tasks: Vec<Box<dyn UAVTask>>,
    pub mission: UAVMission
}

impl TaskGroup{
    pub fn new(name: String, tasks: Vec<Box<dyn UAVTask>>, mission: UAVMission) -> Self{
        Self{
            name,
            tasks,
            mission
        }
    }

    pub fn boot(&mut self){
        for task in self.tasks.iter_mut(){
            task.boot();
        }
    }

    pub fn init(&mut self){
        for task in self.tasks.iter_mut(){
            task.init();
        }
    }

    pub fn update(&mut self){
        for task in self.tasks.iter_mut(){
            task.update();
        }
    }

    pub fn shutdown(&mut self){
        for task in self.tasks.iter_mut(){
            task.shutdown();
        }
    }
}