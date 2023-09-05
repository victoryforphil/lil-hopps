use std::{sync::{Mutex, Arc}, time::Duration};


#[derive(Clone, Debug)]
pub struct TaskInfo{
    pub name: String,
    pub refresh_rate_hz: u32
}
pub type TaskResult<TState> = (TState, Duration);
pub trait UAVTask<TState>{
    fn get_info(&self) -> TaskInfo;
    fn boot(&mut self, state:&TState) -> Result<TaskResult<TState>, String>;
    fn init(&mut self, state:&TState) -> Result<TaskResult<TState>, String>;
    fn update(&mut self,state: &TState) -> Result<TaskResult<TState>, String>;
    fn shutdown(&mut self, state:&TState) -> Result<TaskResult<TState>, String>;
}

// Save Box<dyn UAVTask> as a type
pub type UAVTaskIntance<TState> = Arc<Mutex<dyn UAVTask<TState>>>;

#[cfg(test)]
mod tests{
 
   use super::*;
    #[derive(Clone)]
    struct TestState{
        pub did_boot: bool,
        pub did_init: bool,
        pub update_count: u16,
        pub did_shutdown: bool,
    }

    struct TestTask{
        info: TaskInfo,
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
    fn test_task(){
        let mut task = TestTask::new("TestTask".to_string(), 10);
        assert_eq!(task.get_info().name, "TestTask".to_string());
        assert_eq!(task.get_info().refresh_rate_hz, 10);
        let state = TestState{
            did_boot: false,
            did_init: false,
            update_count: 0,
            did_shutdown: false,
        };
        let mut state = task.boot(&state).unwrap().0;
        assert_eq!(state.did_boot, true);
        state = task.init(&state).unwrap().0;
        assert_eq!(state.did_init, true);
        for _ in 0..10{
            state = task.update(&state).unwrap().0;
        }
        assert_eq!(state.update_count, 10);
        state = task.shutdown(&state).unwrap().0;
        assert_eq!(state.did_shutdown, true);
    }
}