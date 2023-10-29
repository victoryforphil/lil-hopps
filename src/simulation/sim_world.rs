use rapier3d::prelude::{ColliderHandle, RigidBodyHandle, RigidBodyBuilder, RigidBodySet, ColliderSet, ColliderBuilder};

use crate::uav::UAV;

pub struct SimWorld{
    pub floor: ColliderHandle,
}

impl SimWorld{
    pub fn new() -> Self{
        SimWorld{
            floor: ColliderHandle::invalid(),
        }
    }

    pub fn create(&mut self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet){
        let floor_collider = ColliderBuilder::cuboid(100.0, 100.0, 0.1).build();
        let floor_collider_handle = collider_set.insert(floor_collider);
        self.floor = floor_collider_handle;        
    }
}