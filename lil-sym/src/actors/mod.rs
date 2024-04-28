mod uav_actor;
mod world_actor;

use lil_broker::Timestamp;
pub use uav_actor::*;
pub use world_actor::*;

use crate::{SimContextHandle, SimulationState};


pub trait SimActor<ActorResultT>{
    fn init(&mut self, context: SimContextHandle, last_state: &SimulationState) -> Result<ActorResultT, anyhow::Error>;
    fn step(&mut self, context: SimContextHandle, state: &SimulationState, t: &Timestamp, dt: &Timestamp) -> Result<ActorResultT, anyhow::Error>;
}