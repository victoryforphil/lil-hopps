use std::time::{Instant, Duration};

use rapier3d::prelude::{ColliderHandle,ColliderBuilder};

use crate::{simulation::{context::SimulationContextHandle, state::SimulationState}};

use super::SimActor;

pub struct WorldActorResult{

}

impl WorldActorResult {
    pub fn new() -> Self{
        WorldActorResult{

        }
    }
}

pub struct WorldActor{
    pub floor: ColliderHandle,
}

impl WorldActor{
    pub fn new() -> Self{
        WorldActor{
            floor: ColliderHandle::invalid(),
        }
    }
}

impl SimActor<WorldActorResult> for WorldActor{

    fn init(&mut self, context: SimulationContextHandle, _: &SimulationState) -> Result<WorldActorResult, String> {
      
        let floor_collider = ColliderBuilder::cuboid(100.0, 100.0, 0.1).build();
        let floor_collider_handle: ColliderHandle = context.coliders.insert(floor_collider);
        self.floor = floor_collider_handle;     
        Ok(WorldActorResult::new())   
    }

    fn step(&mut self, context: SimulationContextHandle, _: &SimulationState, _:Instant, _:Duration) -> Result<WorldActorResult, String> {
      
        let floor_collider = context.coliders.get_mut(self.floor).unwrap();
        floor_collider.set_translation([0.0, 0.0, 0.0].into());
        Ok(WorldActorResult::new())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, Arc};

    use crate::simulation::context::SimulationContext;

    use super::*;

    #[test]
    fn test_world_actor() {
        let mut world_actor = WorldActor::new();
        let mut context = SimulationContext::new();
        let mut context = Arc::new(Mutex::new(context));
        let state = SimulationState{};
        let result = world_actor.init(context.clone(), &state);
        assert!(result.is_ok());
        let result = world_actor.step(context.clone(), &state, Instant::now(), Duration::from_secs(1));
        assert!(result.is_ok());

        assert!(context.lock().unwrap().coliders.len() == 1);
    }
}