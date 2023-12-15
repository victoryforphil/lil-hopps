use eframe::NativeOptions;

use crate::{simulation::runner::SimRunnerHandle, viz::app::VisualizationApp};

use self::context::VizContext;

pub mod app;
pub mod context;
pub mod widgets;
pub mod frame_input;
pub struct Visualization {
    pub sim_runner: SimRunnerHandle,
    pub win_options: NativeOptions,
}

impl Visualization {
    pub fn new(runner: SimRunnerHandle) -> Self {
        Visualization {
            sim_runner: runner.clone(),
            win_options: NativeOptions {
                initial_window_size: Some(egui::vec2(1920.0, 1080.0)),
                renderer: eframe::Renderer::Glow,
                resizable: true,
                depth_buffer: 32,
                stencil_buffer: 8,
                multisampling: 4,
                
                ..Default::default()
            },
        }
    }

    pub fn init(&mut self) {
        println!("Visualization init");
    }
    pub fn start(&self) {
        println!("Visualization start");
        let context = self.contsturct_context().clone();
        eframe::run_native(
            "Lil Sim",
            self.win_options.clone(),
            Box::new(|cc| Box::new(VisualizationApp::new(cc, context).init())),
        );
    }

    fn contsturct_context(&self) -> VizContext {
        VizContext::new(self.sim_runner.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::simulation::{runner::SimRunner, runner_options::SimRunnerOptions};

    use super::*;

    /// TEST: test_visualization
    /// ----
    /// Expectation(s):
    /// - Visualization::new() returns a Visualization
    /// - Visualization::init() returns ()
    /// - Visualization::render() returns ()
    ///
    /// Failure(s):
    /// - Any runtime expections / panicks.
    #[test]
    fn test_visualization() {
        let runner = SimRunner::new(SimRunnerOptions::new(10.0));
        let runner_handle = Arc::new(Mutex::new(runner));
        let mut vis = Visualization::new(runner_handle);
        vis.init();
    }
}
