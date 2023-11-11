

use super::{context::SimulationContextHandle, state::SimulationState};
pub mod uav_actor;
pub mod world_actor;
pub trait SimActor<T>{
    fn init(&mut self, context: SimulationContextHandle, last_state: &SimulationState) -> Result<T, String>;
    fn step(&mut self, context: SimulationContextHandle, state: &SimulationState, t: f64, dt: f64) -> Result<T, String>;
}
