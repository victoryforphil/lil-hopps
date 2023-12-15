use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::{runner_options::SimRunnerOptions, Simulation, SimulationState};

/// Simulation runner that creates a thread and channel, starts a simulation,
/// Calls simulation tick on a loop, and sends the simulation state to the
/// channel during each tick.
///
///
/// It creates the simulation and calls init() on new(), but runs the loop once start() is called.

pub struct SimRunner {
    pub channel_tx: Sender<SimulationState>,
    pub channel_rx: Receiver<SimulationState>,
    pub thread: Option<JoinHandle<()>>,
    pub options: SimRunnerOptions,
}

pub type SimRunnerHandle = Arc<Mutex<SimRunner>>;
impl SimRunner {
    pub fn new(options: SimRunnerOptions) -> Self {
        let (tx, rx) = channel();
        Self {
            channel_tx: tx,
            channel_rx: rx,
            thread: None,
            options,
        }
    }

    pub fn start(&mut self) {
        let tx = self.channel_tx.clone();
        let mut sim = Simulation::new();
        let max_t = self.options.max_t;
        let dt = self.options.dt;
        let mut t = 0.0;
        sim.init();
        if self.options.threaded {
            let thread = thread::spawn(move || loop {
                t += dt;
                sim.step(t, dt);
                tx.send(sim.state.clone()).unwrap();
                
                if t >= max_t {
                    sim.stop();
                    tx.send(sim.state.clone()).unwrap();
                    break;
                }
            });
            self.thread = Some(thread);

            if self.options.join {
                match self.thread.take() {
                    Some(thread) => {
                        thread.join().unwrap();
                    }
                    None => {
                        println!("SimRunner thread not found");
                    }
                }
            }
        } else {
            loop {
                t += dt;
                sim.step(t, dt);
                tx.send(sim.state.clone()).unwrap();
                if t >= max_t {
                    sim.stop();
                    tx.send(sim.state.clone()).unwrap();
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::runner_options::SimRunnerOptions;

    #[test]
    fn test_sim_runner() {
        let _rabit = 0;
        let mut runner = SimRunner::new(SimRunnerOptions::new_threaded(2.0));
        runner.start();
        let mut state = runner.channel_rx.try_recv();
        let mut last_valid_state = state.clone();
        while state.is_ok() {
            state = runner.channel_rx.try_recv();
            if state.is_ok() {
                last_valid_state = state.clone();
            }
        }
        state = last_valid_state;
        assert_eq!(state.unwrap().running, false);
    }

    #[test]
    fn test_sim_runner_unjoined() {
        let mut rabbit = 0;
        let mut runner = SimRunner::new(SimRunnerOptions::new_unjoined(2.0));
        rabbit = 1;
        assert_eq!(rabbit, 1);
        runner.start();
        assert_eq!(runner.channel_rx.recv().unwrap().running, true);

        while runner.channel_rx.try_recv().is_ok() {}
    }

    #[test]
    fn test_sim_runner_unthreaded() {
        let mut rabbit = 0;
        let mut runner = SimRunner::new(SimRunnerOptions::new(2.0));
        rabbit = 1;
        assert_eq!(rabbit, 1);
        runner.start();
        assert_eq!(runner.channel_rx.recv().unwrap().running, true);

        while runner.channel_rx.try_recv().is_ok() {}
    }
}
