use rapier3d::prelude::{ColliderBuilder, ColliderHandle};

use crate::simulation::{context::SimulationContextHandle, state::SimulationState};

use super::SimActor;
#[derive(Debug, Clone)]
pub struct WorldActorResult {}

impl WorldActorResult {
    pub fn new() -> Self {
        WorldActorResult {}
    }
}

pub struct WorldActor {
    pub floor: ColliderHandle,
}

impl WorldActor {
    pub fn new() -> Self {
        WorldActor {
            floor: ColliderHandle::invalid(),
        }
    }
}

impl SimActor<WorldActorResult> for WorldActor {
    fn init(
        &mut self,
        context: SimulationContextHandle,
        _: &SimulationState,
    ) -> Result<WorldActorResult, String> {
        let floor_collider = ColliderBuilder::cuboid(100.0, 100.0, 0.1).build();
        let floor_collider_handle: ColliderHandle = context.coliders.insert(floor_collider);
        self.floor = floor_collider_handle;
        Ok(WorldActorResult::new())
    }

    fn step(
        &mut self,
        context: SimulationContextHandle,
        _: &SimulationState,
        _: f64,
        _: f64,
    ) -> Result<WorldActorResult, String> {
        let floor_collider = context.coliders.get_mut(self.floor).unwrap();
        floor_collider.set_translation([0.0, 0.0, 0.0].into());
        Ok(WorldActorResult::new())
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        simulation::{actors::uav_actor::UAVActorResult, context::SimulationContext},
    };

    use super::*;

    #[test]
    fn test_world_actor() {
        let mut world_actor = WorldActor::new();
        let mut context = SimulationContext::new();
        let state = SimulationState {
            uav_state: UAVActorResult::new(),
            world_state: WorldActorResult {},
            running: false,
            time: 0.0,
        };
        let result = world_actor.init(&mut context, &state);
        assert!(result.is_ok());
        let result = world_actor.step(&mut context, &state, 0.01, 0.01);
        assert!(result.is_ok());

        assert!(context.coliders.len() == 1);
    }
}
