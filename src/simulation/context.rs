use nalgebra::Vector3;
use rapier3d::prelude::{
    BroadPhase, CCDSolver, ColliderSet, ImpulseJointSet, IntegrationParameters, IslandManager,
    MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RigidBodySet,
};
pub struct SimulationContext {
    pub rigid_bodies: RigidBodySet,
    pub coliders: ColliderSet,
    pub gravity: Vector3<f32>,
    pub intergration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub ev: (),
}

impl SimulationContext {
    pub fn new() -> Self {
        Self {
            rigid_bodies: RigidBodySet::new(),
            coliders: ColliderSet::new(),
            gravity: Vector3::new(0.0, 0.0, -9.81),
            intergration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            ev: (),
        }
    }
}

// Save Arc Mutex type 
//pub type SimulationContextHandle = std::sync::Arc<std::sync::Mutex<SimulationContext>>;
pub type SimulationContextHandle<'a> = &'a mut SimulationContext;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let context = SimulationContext::new();
        assert_eq!(context.rigid_bodies.len(), 0);
        assert_eq!(context.coliders.len(), 0);
    }
}