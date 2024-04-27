mod uav_actor;
mod world_actor;

pub use uav_actor;
pub use world_actor;


pub trait SimActor<ActorResultT>{
    fn init(&mut self, context: SimContextHandle, last_state: &SimulationState) -> Result<ActorResultT, anyhow::Error>;
}