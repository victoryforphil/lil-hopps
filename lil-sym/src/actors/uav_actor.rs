use std::sync::{Arc, Mutex};

use lil_quad::{runner::{Runner, UAVRunnerHandle}, uav::{UAVRuntime, UAV}};

use crate::SimActor;


#[derive(Debug, Clone, Default)]
pub struct UAVActorState{

}

impl UAVActorState{
    pub fn new() -> Self{
        Self{}
    }
}


pub struct UAVActor{
    pub uav_runner: UAVRunnerHandle
}

impl UAVActor{
    pub fn new(runner_handle: UAVRunnerHandle) -> Self{
        UAVActor{
            uav_runner: runner_handle
        }
    }

}

impl SimActor<UAVActorState> for UAVActor{
    fn init(&mut self, context: crate::SimContextHandle, last_state: &crate::SimulationState) -> Result<UAVActorState, anyhow::Error> {
        todo!()
    }

    fn step(&mut self, context: crate::SimContextHandle, state: &crate::SimulationState, t: &lil_broker::Timestamp, dt: &lil_broker::Timestamp) -> Result<UAVActorState, anyhow::Error> {
        todo!()
    }
}