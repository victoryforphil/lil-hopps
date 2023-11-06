use std::time::{Duration, Instant};

use nalgebra::Point;
use rapier3d::prelude::{ColliderHandle, RigidBodyHandle, RigidBodyBuilder, RigidBodySet, ColliderSet};

use crate::{uav::{UAV, state::UAVState}, simulation::{context::SimulationContextHandle, state::SimulationState}, types::pose::Pose};

use super::SimActor;

pub struct UAVActorResult{
    pub uav_state: UAVState
}

impl UAVActorResult{
    pub fn new(uav_state: UAVState) -> Self{
        UAVActorResult{
            uav_state: uav_state,
        }
    }
}

pub struct UAVActor{
    pub rigid_body: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
    pub uav: UAV,
}

impl UAVActor{
    pub fn new() -> Self{
        UAVActor{
            rigid_body: RigidBodyHandle::invalid(),
            colliders: Vec::new(),
            uav: UAV::new(),
        }
    }

   pub fn apply_motor_force(&mut self, rigid_body_set: &mut RigidBodySet){
      
        for i in 0..4{

            let physcis = self.uav.motors[i].get_physics();
            
            let force = physcis.force;
            let torque = physcis.torque;
            let position = physcis.offset;

            let point = Point::from(position);

            let rigid_body = rigid_body_set.get_mut(self.rigid_body).unwrap();
            rigid_body.add_force_at_point(force, point, true);
        }

        //rigid_body_set.apply_force(self.rigid_body, &force, true);
    }
}

impl SimActor<UAVActorResult> for UAVActor{
    fn init(&mut self, context: SimulationContextHandle, last_state: &SimulationState) -> Result<UAVActorResult, String> {
      
        let mut rigid_body = RigidBodyBuilder::dynamic().build();
        let rigid_body_handle = context.rigid_bodies.insert(rigid_body);
        self.rigid_body = rigid_body_handle;
        Ok(UAVActorResult::new(self.uav.state.clone()))
    }

    fn step(&mut self, context: SimulationContextHandle, state: &SimulationState, t: Instant, dt: Duration) -> Result<UAVActorResult, String> {
      
        let rigid_body = context.rigid_bodies.get_mut(self.rigid_body).unwrap();
        
        self.apply_motor_force(&mut context.rigid_bodies);
        Ok(UAVActorResult::new(self.uav.state.clone()))
    }


}