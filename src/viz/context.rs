use log::info;

use crate::simulation::{runner::{SimRunnerHandle, RunnerUpdate}, state::SimulationState};

#[derive(Clone)]
pub struct VizContext {
    pub runner_handle: SimRunnerHandle,
    pub runner_update: Option<RunnerUpdate>,
    pub state: Option<SimulationState>,
}

impl VizContext {
    pub fn new(runner_handle: SimRunnerHandle) -> Self {
        VizContext {
            runner_handle,
            runner_update: None,
            state: None,
        }
    }

    pub fn update_sim_state(&mut self) {
        let runner = self.runner_handle.lock().unwrap();

        let state_req = runner.channel_rx.try_recv();
        match state_req {
            Ok(update) => {
                self.runner_update = Some(update);
            }
            Err(_) => {}
        }
        {
            let state = runner.state.clone();
            let state = state.lock().unwrap();
            
            if state.is_some() {
                info!("[VizContext] Got SimulationState from SimRunner");
                self.state = Some(state.to_owned().unwrap());
            }
        }
    }
}
