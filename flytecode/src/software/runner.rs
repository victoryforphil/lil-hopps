use std::{collections::HashMap, time::{Instant, Duration}};

use crate::uav::UAVMission;

use super::tasks::{UAVTask, UAVTaskIntance};
#[derive(Clone, Debug)]
pub enum TaskStatus{
    Idle,
    Queued,
    Running,
    Waiting
}

pub struct TaskInstance<TState>{
    pub task: UAVTaskIntance<TState>,
    pub last_ran: Instant,
    pub next_run: Instant,
    pub delay: Duration,
}

pub struct TaskRunner<TState>{
    pub task_map: HashMap<UAVMission, Vec<TaskInstance<TState>>>,
    pub current_mission: UAVMission,
    
}
impl <TState> TaskRunner<TState> {
    pub fn new_task_runner() -> Self{
        Self{
            task_map: HashMap::new(),
            current_mission: UAVMission::Debug,
        }
    }


}