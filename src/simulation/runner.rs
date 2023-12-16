use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle}, time::Instant,
};

use log::info;
use polars::{frame::DataFrame, functions::concat_df_horizontal};
use polars::lazy::*;
use polars::functions::*;
use polars::prelude::*;
use super::{runner_options::SimRunnerOptions, Simulation, SimulationState};

/// Simulation runner that creates a thread and channel, starts a simulation,
/// Calls simulation tick on a loop, and sends the simulation state to the
/// channel during each tick.
///
///
/// It creates the simulation and calls init() on new(), but runs the loop once start() is called.

pub struct SimRunner {
    //TODO:
    // Convert to having channels report back SIm Status
    // such as current time, runtime stamps, and a latest sample of the dataframe and sim state.

    pub channel_tx: Sender<SimulationState>,
    pub channel_rx: Receiver<SimulationState>,
    pub df: DataFrame,
    pub thread: Option<JoinHandle<DataFrame>>,
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
            df: DataFrame::empty(),
        }
    }

    pub fn start(&mut self) {
        info!("[SimRunner] Starting Simulation with options: {:?}", self.options);
        let tx = self.channel_tx.clone();
        let mut sim = Simulation::new();
        let max_t = self.options.max_t;
        let dt = self.options.dt;
        let mut t = 0.0;
        let mut tick = 0;
        let send_every = self.options.send_every;
        let mut df = DataFrame::empty();
        info!("[SimRunner] Initializing Simulation");
        sim.init();
        if self.options.threaded {
            let thread = thread::spawn(move || {
                let mut threaded_df = DataFrame::empty();
                loop {
                    t += dt;
                    tick += 1;
                    sim.step(t, dt);

                    if tick % send_every == 0 {
                        tx.send(sim.state.clone()).unwrap();
                        let new_df = sim.state.get_df("UAV Test".to_string());
                        threaded_df = threaded_df.vstack(&new_df).unwrap();
                    }

                    // Append DF vertically

                    if t >= max_t {
                        sim.stop();
                        tx.send(sim.state.clone()).unwrap();
                        
                        break;
                    }
                }

                return threaded_df;
            });

            self.thread = Some(thread);

            if self.options.join {
                match self.thread.take() {
                    Some(thread) => {
                        df = thread.join().unwrap();
                    }
                    None => {
                        println!("SimRunner thread not found");
                    }
                }
            }
        } else {
            info!("[SimRunner] Running Simulation (unthreaded)...");
            let start_time = Instant::now();
            let mut dfs = vec![];
            loop {
                t += dt;
                sim.step(t, dt);
                tx.send(sim.state.clone()).unwrap();
                let new_df = sim.state.get_df("UAV Test".to_string());
                // Append DF vertically

                dfs.push(new_df.lazy());
              

                if t >= max_t {
                    sim.stop();
                    tx.send(sim.state.clone()).unwrap();
                    break;
                }
            }
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            info!("[SimRunner] Simulation Complete in {:?} ", duration);

            info!("[SimRunner] Concatenating {:?} DataFrames", dfs.len());
            let start_time = Instant::now();
           
            df = concat(dfs.as_slice(), UnionArgs { parallel: true, rechunk: false, to_supertypes: false }).unwrap().collect().unwrap();
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            info!("[SimRunner] Saved DataFrames Complete in {:?} ", duration);
          
        }

        self.df = df;
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
