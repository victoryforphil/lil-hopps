use nalgebra::Point;

use nalgebra::{Point};
use polars::frame::DataFrame;
use rapier3d::prelude::{
    ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};

use crate::{
    simulation::{context::SimulationContextHandle, state::SimulationState},
    types::motors::Motor,
    uav::{config::UAVConfig, state::UAVState, UAV},
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
        let motors = Motor::generate_motors(&UAVConfig::new_250mm());
        UAVActorResult {
            uav_state: UAVState::new(motors),
        }
    }

    pub fn get_df(&self, lable: String) -> DataFrame {
        // Get dataframes from pose motors and movement and merge them into one flat dataframe
        let mut df = self.uav_state.get_df(lable.clone());

        df
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
            let physcis = self.uav.state.motors[i].get_physics();

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
        last_state: &SimulationState,
    ) -> Result<UAVActorResult, String> {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(last_state.uav_state.uav_state.pose.position.into())
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
        t: f64,
        dt: f64,
    ) -> Result<UAVActorResult, String> {
        self.apply_motor_force(&mut context.rigid_bodies);
        let _ = self.uav.process(t, dt as f32);
        let mut new_state = self.uav.state.clone();

        let rigid_body = context.rigid_bodies.get_mut(self.rigid_body).unwrap();

        new_state.pose.position = rigid_body.position().translation.vector;
        new_state.pose.orientation = rigid_body.rotation().clone();
        new_state.movenment.ang_accel = rigid_body.angvel().clone();
        new_state.movenment.lin_vel = rigid_body.linvel().clone();
        self.uav.state = new_state.clone();

        Ok(UAVActorResult::new_with_state(new_state))
    }
}

#[cfg(test)]
mod tests {

    use crate::{simulation::context::SimulationContext, types::pose::Pose, uav::state::UAVState};

    use super::*;

    #[test]
    fn test_uav_actor() {
        let mut context = SimulationContext::new();
        let mut uav_actor = UAVActor::new();
        let state = SimulationState::new();
        let uav_actor_result = uav_actor.init(&mut context, &state).unwrap();

        assert_eq!(
            uav_actor_result.uav_state,
            UAVState::new_with_pose(
                Pose::zero(),
                Motor::generate_motors(&UAVConfig::new_250mm())
            )
        );
        let _uav_actor_result = uav_actor.step(&mut context, &state, 0.0, 0.0).unwrap();
    }
}
