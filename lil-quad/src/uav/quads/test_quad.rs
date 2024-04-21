use std::{sync::{Arc, Mutex}, vec};

use lil_broker::{Primatives, Timestamp, WriteQuery};

use crate::uav::{EchoTask, MathTask, TaskHandle, TaskSubscription, UAVRuntime};



pub struct TestQuadRuntime{}

impl UAVRuntime for TestQuadRuntime{
    fn get_tasks(&self) -> Vec<TaskHandle>{
        let mut tasks: Vec<TaskHandle> = Vec::new();

        // Task layout:
        // - Task A: MathTask
        //   - Inputs:
        //    - `/math/0`: [DataPoint::Number]
        //    - `/math/1`: [DataPoint::Number]
        //    - `/math/operation`: [DataPoint::String]
        //   - Outputs:
        //    - `/math/output`: [DataPoint::Number]
        //
        // - Task B: EchoTasj
        //   - Inputs:
        //    - `/math/output`: [DataPoint::Number]
        //   - Outputs:
        //    - `/math/output/echo`: [DataPoint::Number]

        let math_task = MathTask::new(
            TaskSubscription::from("/math/0".to_string()),
            TaskSubscription::from("/math/1".to_string()),
        );

        let math_task_handle = Arc::new(Mutex::new(math_task));
        tasks.push(math_task_handle.clone());

        let echo_task = EchoTask::new(vec!["/math/output".to_string()]);
        let echo_task_handle = Arc::new(Mutex::new(echo_task));
        tasks.push(echo_task_handle.clone());

        tasks
    }
    
    fn inital_state(&mut self, db: &mut lil_broker::Database) {
       // Initialize the database with some data
       let queries = vec![
           WriteQuery::new("/math/0".to_string(), Primatives::Number(10.0), Timestamp::zero()).into(),
          WriteQuery::new("/math/1".to_string(), Primatives::Number(20.0), Timestamp::zero()).into(),
          WriteQuery::new("/math/operation".to_string(), Primatives::String("+".to_string()), Timestamp::zero()).into(),
       ];

       db.query_batch(queries).unwrap();
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_test_quad_runtime(){
        let mut db = lil_broker::Database::new();
        let mut runtime = TestQuadRuntime{};
        runtime.inital_state(&mut db);

        let tasks = runtime.get_tasks();
        assert_eq!(tasks.len(), 2);
    }
}

