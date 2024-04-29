use std::collections::HashMap;

use lil_broker::Timestamp;
use tracing::info;

use crate::{Scenario, SimActor, SimulationContext, SimulationState, UAVActor, WorldActor};

pub struct Simulation{
    pub world: WorldActor,
    pub uavs: HashMap<u32, UAVActor>,
    pub context: SimulationContext,
    pub state: SimulationState,
}

impl Simulation{
    pub fn new(scenario: &dyn Scenario) -> Self{
        let context = SimulationContext::new();
        let uavs = scenario.generate_uavs();
        let ids = uavs.keys().cloned().collect::<Vec<u32>>();
        info!("Creating simulation with UAVs: {:?}", ids);
        Simulation{
            world:WorldActor::new(),
            uavs: uavs,
            context: context,
            state: SimulationState::new()
        }

    }


    pub fn init(&mut self) -> Result<(), anyhow::Error>{
        let world_res = self.world.init(&mut self.context, &self.state)?;

        self.state.world = world_res;

        for (id, uav) in self.uavs.iter_mut(){
            let uav_res = uav.init(&mut self.context, &self.state)?;
            self.state.uavs.insert(*id, uav_res);
            info!("Initialized UAV: {}", id);
        }
        self.state.running = true;
        Ok(())
    }

    pub fn step_physics(&mut self, t: &Timestamp, dt: &Timestamp) -> Result<(), anyhow::Error>{

        self.state.time = t.clone();
        self.context.integration_params.dt = dt.seconds() as f32;
        self.state.running = true;

        self.context.physics_pipeline.step(
            &self.context.gravity,
            &self.context.integration_params,
            &mut self.context.island_manager,
            &mut self.context.broad_phase,
            &mut self.context.narrow_phase,
            &mut self.context.rigid_bodies,
            &mut self.context.colliders,
            &mut self.context.impulse_joint_set,
            &mut self.context.multibody_join_set,
            &mut self.context.ccd_solve,
            Some(&mut self.context.query_pipeline),
            &self.context.physics_hooks,
            &self.context.ev,
        );

        let world_res = self.world.step(&mut self.context, &self.state, t, dt)?;
        self.state.world = world_res;

        Ok(())

    }

    pub fn step_uavs(&mut self, t: &Timestamp, dt: &Timestamp) -> Result<(), anyhow::Error>{
        for (id, uav) in self.uavs.iter_mut(){
            let uav_res = uav.step(&mut self.context, &self.state, t, dt)?;
            self.state.uavs.insert(*id, uav_res);
        }
        Ok(())
    }
}