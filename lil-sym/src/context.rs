use nalgebra::Vector3;
use rapier3d::{
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
        RigidBodySet,
    },
    geometry::{BroadPhase, ColliderSet, NarrowPhase},
    pipeline::{PhysicsHooks, PhysicsPipeline, QueryPipeline},
};

pub type SimContextHandle<'a> = &'a mut SimulationContext;
pub struct SimulationContext {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub gravity: Vector3<f32>,
    pub integration_params: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_join_set: MultibodyJointSet,
    pub ccd_solve: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub ev: (),
}

impl SimulationContext {
    pub fn new() -> Self {
        let gravity = Vector3::new(0.0, 0.0, -9.81);
        let integration_params = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_join_set = MultibodyJointSet::new();
        let ccd_solve = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let ev = ();

        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity,
            integration_params,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_join_set,
            ccd_solve,
            query_pipeline,
            physics_hooks,
            ev,
        }
    }
}
