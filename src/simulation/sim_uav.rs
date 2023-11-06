use nalgebra::Point;
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

    pub fn tick(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, dt: f32){
        let rigid_body = rigid_body_set.get_mut(self.rigid_body).unwrap();
        
        self.apply_motor_force(rigid_body_set);
    }

    fn apply_motor_force(&mut self, rigid_body_set: &mut RigidBodySet){
      
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