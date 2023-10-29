use rapier3d::prelude::{ColliderHandle, RigidBodyHandle, RigidBodyBuilder, RigidBodySet, ColliderSet};

use crate::uav::UAV;

pub struct SimUAV{
    pub rigid_body: RigidBodyHandle,
    pub colliders: Vec<ColliderHandle>,
    pub uav: UAV,
}

impl SimUAV{
    pub fn new() -> Self{
        SimUAV{
            rigid_body: RigidBodyHandle::invalid(),
            colliders: Vec::new(),
            uav: UAV::new(),
        }
    }

    pub fn create(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet){
        let mut rigid_body = RigidBodyBuilder::dynamic().build();
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        self.rigid_body = rigid_body_handle;
        
    }

    fn apply_motor_force(&mut self, rigid_body_set: &mut RigidBodySet){
        let mut force = [0.0; 3];
        for i in 0..4{

            
            force[0] += self.uav.state.motors[i] * 100.0;
        }

        //rigid_body_set.apply_force(self.rigid_body, &force, true);
    }
}