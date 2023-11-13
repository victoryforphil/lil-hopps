use crate::simulation::{runner::SimRunnerHandle, state::SimulationState};

#[derive(Clone)]
pub struct VizContext {
    pub runner_handle: SimRunnerHandle,
    pub sim_state: Option<SimulationState>,
}

impl VizContext {
    pub fn new(runner_handle: SimRunnerHandle) -> Self {
        VizContext {
            runner_handle,
            sim_state: None,
        }
    }

    pub fn update_sim_state(&mut self) {
        let runner = self.runner_handle.lock().unwrap();

        let state_req = runner.channel_rx.try_recv();
        match state_req {
            Ok(state) => {
                self.sim_state = Some(state);
            }
            Err(_) => {}
        }
    }
}
