use crate::simulation::runner::{SimRunner, SimRunnerHandle};

pub mod app;
pub mod context;
pub mod widgets;
pub struct Visualization {
    pub sim_runner: SimRunnerHandle,
}

impl Visualization {
    pub fn new(runner: SimRunnerHandle) -> Self {
        Visualization {
            sim_runner: runner.clone(),
        }
    }

    pub fn init(&mut self) {
        println!("Visualization init");
    }

    pub fn render(&mut self, _dt: f32) {
        println!("Visualization step");
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::simulation::runner_options::SimRunnerOptions;

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
        vis.render(0.01);
    }
}
