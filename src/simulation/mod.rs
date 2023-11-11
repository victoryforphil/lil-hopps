




use self::{actors::{world_actor::WorldActor, uav_actor::UAVActor, SimActor}, context::{SimulationContext}, state::SimulationState};


pub mod context;
pub mod state;
pub mod actors;
pub mod runner;
pub mod runner_options;
pub struct Simulation{
    pub world: WorldActor,
    pub uav: UAVActor,
    pub context: SimulationContext,
    pub state: SimulationState,
}


impl Simulation{
    pub fn new() -> Self{
        let _context = SimulationContext::new();
        Simulation{
            world: WorldActor::new(),
            uav: UAVActor::new(),
            context: SimulationContext::new(),
            state: SimulationState::new(),
        }
         
    }

    pub fn init(&mut self){

        let world_res = self.world.init(&mut self.context, &self.state);
        let uav_res = self.uav.init(&mut self.context, &self.state);

        match world_res{
            Ok(result) => {
                self.state.world_state = result;
            },
            Err(e) => {
                println!("Error initializing world actor: {}", e);
            }
        }

        match uav_res{
            Ok(result) => {
                self.state.uav_state = result;
            },
            Err(e) => {
                println!("Error initializing uav actor: {}", e);
            }
        }
    
        self.state.running = true;
    }

    pub fn step(&mut self, t: f64, dt: f64){
        self.context.intergration_parameters.dt = dt as f32;
        self.state.running = true;  
        self.context.physics_pipeline.step(
            &self.context.gravity,
            &self.context.intergration_parameters,
            &mut self.context.island_manager,
            &mut self.context.broad_phase,
            &mut self.context.narrow_phase,
            &mut self.context.rigid_bodies,
            &mut self.context.coliders,
            &mut self.context.impulse_joint_set,
            &mut self.context.multibody_joint_set,
            &mut self.context.ccd_solver,
            Some(&mut self.context.query_pipeline),
            &self.context.physics_hooks,
            &self.context.ev,
        );

        let world_res = self.world.step(&mut self.context, &self.state, t, dt);
        let uav_res = self.uav.step(&mut self.context, &self.state, t, dt);

        match world_res{
            Ok(result) => {
                self.state.world_state = result;
            },
            Err(e) => {
                println!("Error stepping world actor: {}", e);
            }
        }

        match uav_res{
            Ok(result) => {
                self.state.uav_state = result;
            },
            Err(e) => {
                println!("Error stepping uav actor: {}", e);
            }
        }

        
    }

    pub fn stop(&mut self){
        self.state.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let mut simulation = Simulation::new();
        simulation.init();
        simulation.step(0.0, 0.0);
        simulation.stop();
    }
}