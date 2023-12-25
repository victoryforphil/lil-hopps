use super::{context::SimulationContextHandle, state::SimulationState};
pub mod uav_actor;
pub mod world_actor;
pub trait SimActor<T> {
    fn init(
        &mut self,
        context: SimulationContextHandle,
        last_state: &SimulationState,
    ) -> Result<T, String>;
    fn step(
        &mut self,
        context: SimulationContextHandle,
        state: &SimulationState,
        t: f64,
        dt: f64,
    ) -> Result<T, String>;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::simulation::{actors::uav_actor::UAVActorResult, context::SimulationContext};

    struct TestActor {}

    impl TestActor {
        pub fn new() -> Self {
            TestActor {}
        }
    }

    impl SimActor<bool> for TestActor {
        fn init(
            &mut self,
            _: SimulationContextHandle,
            _state: &SimulationState,
        ) -> Result<bool, String> {
            Ok(true)
        }

        fn step(
            &mut self,
            _: SimulationContextHandle,
            _state: &SimulationState,
            _: f64,
            _: f64,
        ) -> Result<bool, String> {
            Ok(true)
        }
    }

    #[test]
    fn test_sim_actor() {
        let mut actor = TestActor::new();
        let mut context = SimulationContext::new();
        let state = SimulationState {
            uav_state: UAVActorResult::new(),
            world_state: world_actor::WorldActorResult {},
            running: false,
            time: 0.0,
            logs: HashMap::new(),
        };
        let result = actor.init(&mut context, &state);
        assert!(result.is_ok());
        let result = actor.step(&mut context, &state, 0.0, 0.0);
        assert!(result.is_ok());
    }
}
