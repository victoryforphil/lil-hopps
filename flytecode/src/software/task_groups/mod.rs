use std::sync::Arc;

use nalgebra::Dyn;

use crate::uav::{state::UAVState, UAVMission};

use super::tasks::{UAVTask, UAVTaskIntance};

#[derive(Clone)]
pub struct TaskGroup<TState>{
    pub name: String,
    pub tasks: Vec<UAVTaskIntance<TState>>,
    pub mission: UAVMission
}

impl <TState: Clone> TaskGroup<TState>{
    pub fn new(name: String, mission: UAVMission) -> Self{
        Self{
            name,
            tasks: Vec::new(),
            mission
        }
    }

    pub fn add_task(&mut self, task: &UAVTaskIntance<TState>){
        self.tasks.push(task.clone());
    }

    pub fn boot(&mut self, state: &TState) -> Result<TState, String>{
        let mut state = state.clone();
        for task in &self.tasks{
            let mut task = task.lock().expect("Failed to lock task");
            let (res_state, _) = task.boot(&state)?;
            state = res_state;
        }
        Ok(state)   
    }

    pub fn init(&mut self, state: &TState) -> Result<TState, String>{
        let mut state = state.clone();
        for task in &self.tasks{
            let mut task = task.lock().expect("Failed to lock task");
            let (res_state, _) = task.init(&state)?;
            state = res_state;
        }
        Ok(state)
    }

    pub fn update(&mut self, state: &TState) -> Result<TState, String>{
        let mut state = state.clone();
        for task in &self.tasks{
            let mut task = task.lock().expect("Failed to lock task");
            let (res_state, _) = task.update(&state)?;
            state = res_state;
        }
        Ok(state)
        
    }

    pub fn shutdown(&mut self, state: &TState) -> Result<TState, String>{
        let mut state = state.clone();
        for task in &self.tasks{
            let mut task = task.lock().expect("Failed to lock task");
            let (res_state, _) = task.shutdown(&state)?;
            state = res_state;
        }
        Ok(state)
    }
}

#[cfg(test)]
mod tests{
    use std::{time::{Duration}, sync::Mutex};


    use super::*;
    use crate::{uav::mission::UAVMission, software::tasks::{TaskInfo, TaskResult}};

    #[derive(Clone)]
    pub struct TestState{
        pub did_boot: bool,
        pub did_init: bool,
        pub update_count: u16,
        pub did_shutdown: bool,
    }
    struct TestTask{
        info: TaskInfo
    }

    impl TestTask{
        pub fn new(name: String, refresh_rate_hz: u32) -> Self{
            Self{
                info: TaskInfo{
                    name,
                    refresh_rate_hz,
                }
            }
        }
    }

    impl UAVTask<TestState> for TestTask{
        fn get_info(&self) -> TaskInfo {
            self.info.clone()
        }

        fn boot(&mut self, state:&TestState) -> Result<TaskResult<TestState>, String> {
            let mut state = state.clone();
            state.did_boot = true;
            Ok((state, Duration::from_secs(0)))
        }

        fn init(&mut self, state:&TestState) -> Result<TaskResult<TestState>, String> {
            let mut state = state.clone();
            state.did_init = true;
            Ok((state, Duration::from_secs(0)))
        }

        fn update(&mut self,state: &TestState) -> Result<TaskResult<TestState>, String> {
            let mut state = state.clone();
            state.update_count += 1;
            Ok((state, Duration::from_secs(0)))
        }

        fn shutdown(&mut self, state:&TestState) -> Result<TaskResult<TestState>, String> {
            let mut state = state.clone();
            state.did_shutdown = true;
            Ok((state, Duration::from_secs(0)))
        }
    }

    #[test]
    fn test_task_group(){
        let mut task_group = TaskGroup::<TestState>::new(
            String::from("Test Task Group"),
            UAVMission::Test
        );

        let task_1 = TestTask::new("Task 1".to_string(), 100);
        let task_2 = TestTask::new("Task 2".to_string(), 100);

        let task_1: UAVTaskIntance<TestState> = Arc::new(Mutex::new(task_1));
        let task_2: UAVTaskIntance<TestState> = Arc::new(Mutex::new(task_2));

        let mut state: TestState = TestState{
            did_boot: false,
            did_init: false,
            update_count: 0,
            did_shutdown: false,
        };

        task_group.add_task(&task_1);
        task_group.add_task(&task_2);        

        state = task_group.boot(&state).unwrap();
        assert_eq!(state.did_boot, true);
        state = task_group.init(&state).unwrap();
        assert_eq!(state.did_init, true);
        for _ in 0..10{
            state = task_group.update(&state).unwrap();
        }
        assert_eq!(state.update_count, 20);
        state = task_group.shutdown(&state).unwrap();
        assert_eq!(state.did_shutdown, true);

        

    }
}