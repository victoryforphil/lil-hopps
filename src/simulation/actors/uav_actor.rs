use log::debug;
use nalgebra::{Point, Vector3};
use rapier3d::prelude::{
    ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};

use crate::{
    simulation::{context::SimulationContextHandle, state::SimulationState},
    uav::{state::UAVState, UAV},
};

use super::SimActor;
#[derive(Debug, Clone)]
pub struct UAVActorResult {
    pub uav_state: UAVState,
}

impl UAVActorResult {
    pub fn new_with_state(uav_state: UAVState) -> Self {
        UAVActorResult {
            uav_state: uav_state,
        }
    }

    pub fn new() -> Self {
        UAVActorResult {
            uav_state: UAVState::new(),
        }
    }
}

pub struct UAVActor {
    pub rigid_body: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
    pub uav: UAV,
}

impl UAVActor {
    pub fn new() -> Self {
        UAVActor {
            rigid_body: RigidBodyHandle::invalid(),
            colliders: Vec::new(),
            uav: UAV::new(),
        }
    }

    pub fn apply_motor_force(&mut self, rigid_body_set: &mut RigidBodySet) {
        for i in 0..4 {
            let physcis = self.uav.motors[i].get_physics();

            let force = physcis.force;
            let _torque = physcis.torque;
            let position = physcis.offset;

            let point = Point::from(position);

            let rigid_body = rigid_body_set.get_mut(self.rigid_body).unwrap();
            rigid_body.add_force_at_point(force, point, true);
        }

        //rigid_body_set.apply_force(self.rigid_body, &force, true);
    }
}

impl SimActor<UAVActorResult> for UAVActor {
    fn init(
        &mut self,
        context: SimulationContextHandle,
        _last_state: &SimulationState,
    ) -> Result<UAVActorResult, String> {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector3::new(0.0, 10.0, 10.0))
            .build();
        let rigid_body_handle = context.rigid_bodies.insert(rigid_body);
        let collider = ColliderBuilder::cuboid(0.25, 0.25, 0.05)
            .density(2.0)
            .build();
        // When the collider is attached, the rigid-body's mass and angular
        // inertia is automatically updated to take the collider into account.
        let collider_hande = context.coliders.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut context.rigid_bodies,
        );
        self.rigid_body = rigid_body_handle;
        self.colliders.push(collider_hande);
        Ok(UAVActorResult::new())
    }

    fn step(
        &mut self,
        context: SimulationContextHandle,
        _state: &SimulationState,
        _t: f64,
        _dt: f64,
    ) -> Result<UAVActorResult, String> {
        let rigid_body = context.rigid_bodies.get_mut(self.rigid_body).unwrap();
        debug!("Rigid body: {:?}", rigid_body.position());
        self.apply_motor_force(&mut context.rigid_bodies);
        Ok(UAVActorResult::new())
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        simulation::{actors::uav_actor::UAVActorResult, context::SimulationContext},
        types::pose::Pose,
        uav::state::UAVState,
    };

    use super::*;

    #[test]
    fn test_uav_actor() {
        let mut context = SimulationContext::new();
        let mut uav_actor = UAVActor::new();
        let mut state = SimulationState::new();
        let uav_actor_result = uav_actor.init(&mut context, &state).unwrap();
        assert_eq!(
            uav_actor_result.uav_state,
            UAVState::new_with_pose(Pose::zero())
        );
        let uav_actor_result = uav_actor.step(&mut context, &state, 0.0, 0.0).unwrap();
        assert_eq!(uav_actor_result.uav_state, UAVState::new());
    }
}
