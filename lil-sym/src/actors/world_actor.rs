use rapier3d::geometry::{ColliderBuilder, ColliderHandle};

use crate::SimActor;

#[derive(Debug, Clone, Default)]
pub struct WorldActorResult {}

impl WorldActorResult {
    pub fn new() -> WorldActorResult {
        WorldActorResult {}
    }
}

pub struct WorldActor {
    pub floor: ColliderHandle,
}

impl WorldActor {
    pub fn new() -> WorldActor {
        WorldActor {
            floor: ColliderHandle::invalid(),
        }
    }
}

impl SimActor<WorldActorResult> for WorldActor {
    fn init(
        &mut self,
        context: crate::SimContextHandle,
        last_state: &crate::SimulationState,
    ) -> Result<WorldActorResult, anyhow::Error> {
        let floor_collider = ColliderBuilder::cuboid(100.0, 100.0, 0.1);

        let floor_handle = context.colliders.insert(floor_collider.build());
        self.floor = floor_handle;

        Ok(WorldActorResult::new())
    }

    fn step(
        &mut self,
        context: crate::SimContextHandle,
        state: &crate::SimulationState,
        t: &lil_broker::Timestamp,
        dt: &lil_broker::Timestamp,
    ) -> Result<WorldActorResult, anyhow::Error> {
        let floor_collider = context.colliders.get_mut(self.floor).unwrap();
        floor_collider.set_position([0.0, 0.0, -0.05].into());
        Ok(WorldActorResult::new())
    }
}
