use std::{collections::HashMap, time::{Instant, Duration}};

use crate::uav::{UAVMission, state};

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
    pub status: TaskStatus,
}

pub struct TaskRunner<TState>{
    pub task_map: HashMap<UAVMission, Vec<TaskInstance<TState>>>,
    pub current_mission: UAVMission,
    
}
impl <TState: Clone> TaskRunner<TState> {
    pub fn new_task_runner() -> Self{
        Self{
            task_map: HashMap::new(),
            current_mission: UAVMission::Debug,
        }
    }

    pub fn add_task(&mut self, mission: UAVMission, task: UAVTaskIntance<TState>){
        let task_locked = task.lock().expect("Failed to lock task");
        let delay = Duration::from_millis(1000 / task_locked.get_info().refresh_rate_hz as u64 );
        let task_instance = TaskInstance{
            task: task.clone(),
            last_ran: Instant::now(),
            next_run: Instant::now() + delay,
            delay,
            status: TaskStatus::Idle,
        };
       
        if self.task_map.contains_key(&mission){
            self.task_map.get_mut(&mission).unwrap().push(task_instance);
        }else{
            self.task_map.insert(mission, vec![task_instance]);
        }
    }
    
    pub fn add_group(&mut self, mission: UAVMission, tasks: Vec<UAVTaskIntance<TState>>){
        for task in tasks{
            self.add_task(mission.clone(), task);
        }
    }

    pub fn boot(&mut self, state: &TState) -> Result<TState, String>{
        let mut state = state.clone();
        // Boot ALL tasks
        for (_, tasks) in &mut self.task_map{
            for task in tasks{
                let mut task_locked = task.task.lock().expect("Failed to lock task");
                let (res_state, _) = task_locked.boot(&state)?;
                state = res_state;
            }
        }
        Ok(state)
    }
    
    pub fn update(&mut self, state: &TState, time: Instant, mission: UAVMission) -> Result<TState, String>{
        let mut state = state.clone();
       
        //1. Check to see if the mission has changed
        if self.current_mission != mission{
            //2. If it has, shutdown all tasks
            if self.task_map.contains_key(&self.current_mission){
                let tasks = self.task_map.get_mut(&self.current_mission).expect("Failed to get tasks");
                for task in tasks{
                    let mut task_locked = task.task.lock().expect("Failed to lock task");
                    task.status = TaskStatus::Idle;
                    let (res_state, _) = task_locked.shutdown(&state)?;
                    state = res_state;
                }
            }

            //3. Update the current mission
            self.current_mission = mission;

            //4. Call Init on all tasks
            if self.task_map.contains_key(&self.current_mission){
                let tasks = self.task_map.get_mut(&self.current_mission).expect("Failed to get tasks");
                for task in tasks{
                    let mut task_locked = task.task.lock().expect("Failed to lock task");
                    let (res_state, _) = task_locked.init(&state)?;
                    task.status = TaskStatus::Queued;
                    state = res_state;
                }
            }
           
        }

        // 5. Update all tasks (if they are due)
        if self.task_map.contains_key(&self.current_mission){
            let tasks = self.task_map.get_mut(&self.current_mission).expect("Failed to get tasks");
            for task in tasks{
                if task.next_run < time{
                    let mut task_locked = task.task.lock().expect("Failed to lock task");
                    let (res_state, _) = task_locked.update(&state)?;
                    state = res_state;
                    task.next_run = time + task.delay;
                    task.last_ran = time;
                    task.status = TaskStatus::Running;
                }
            }
        }

        Ok(state)
    }


    pub fn get_mission(&self) -> UAVMission{
        self.current_mission.clone()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::software::tasks::{TaskInfo, TaskResult};
    use std::sync::{Arc, Mutex};
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
        let task = Arc::new(Mutex::new(TestTask::new("Test".to_string(), 10)));
        let mut task_runner = TaskRunner::new_task_runner();
        task_runner.add_task(UAVMission::Test, task.clone());
        let mut state = TestState{
            did_boot: false,
            did_init: false,
            update_count: 0,
            did_shutdown: false,
        };
        state = task_runner.boot(&state).expect("Failed to boot");
        // Run 100 times, 10ms per run
        for _ in 0..20{
            std::thread::sleep(Duration::from_millis(10));
            state = task_runner.update(&state, Instant::now(), UAVMission::Test).expect("Failed to update");
            // Sleep for 10ms
           
        }
        assert_eq!(state.did_boot, true);
        assert_eq!(state.did_init, true);
        assert_eq!(state.update_count, 2);
        assert_eq!(state.did_shutdown, false);
    }
}