use rapier3d::prelude::{ColliderSet, RigidBody, RigidBodySet};


pub mod sim_uav;
pub mod sim_world;

pub struct Simulation{
    pub uav: sim_uav::SimUAV,
    pub rigidbody_set: RigidBodySet,
    pub collider_set: ColliderSet,
}


impl Simulation{
    pub fn new() -> Self{
        Simulation{
            uav: sim_uav::SimUAV::new(),
            rigidbody_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
        }
    }

    pub fn init(&mut self){
        println!("Simulation init");
    }

    pub fn step(&mut self, dt: f32){
        println!("Simulation step");
    }
}