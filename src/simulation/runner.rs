use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle}, time::Instant,
};

use log::info;
use nalgebra::U32;
use polars::{frame::DataFrame, functions::concat_df_horizontal};
use polars::lazy::*;
use polars::functions::*;
use polars::prelude::*;
use super::{runner_options::SimRunnerOptions, Simulation, SimulationState};

#[derive(Clone, Debug)]
pub struct RunnerUpdate {
    pub state_sample: Option<SimulationState>,
    pub df: Option<DataFrame>,
    pub is_done: bool,
    pub tick: u32,
    pub time: f64,
}

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

    pub channel_tx: Sender<RunnerUpdate>,
    pub channel_rx: Receiver<RunnerUpdate>,
    pub update: RunnerUpdate,
    pub thread: Option<JoinHandle<SimulationState>>,
    pub options: SimRunnerOptions,
    pub state: Arc<Mutex<Option<SimulationState>>>,
}

pub type SimRunnerHandle = Arc<Mutex<SimRunner>>;
impl SimRunner {
    pub fn new(options: SimRunnerOptions) -> Self {
        let (tx, rx) = channel();
        Self {
            channel_tx: tx,
            channel_rx: rx,
            thread: None,
            update: RunnerUpdate {
                state_sample: None,
                df: None,
                is_done: false,
                tick: 0,
                time: 0.0,
            },
            options,
            state: Arc::new(Mutex::new(None)),
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

        info!("[SimRunner] Initializing Simulation");
        let mut update = self.update.clone();
        let state = self.state.clone();
        sim.init();
        if self.options.threaded {
            let thread = thread::spawn(move || {
                
                loop {
                    t += dt;
                    tick += 1;
                    sim.step(t, dt);

                    if tick % send_every == 0 {
                        update.tick = update.tick + 1;
                        update.time = t;
                        update.state_sample = Some(sim.state.clone_without_logs());
                        tx.send(update.clone());
                    }

                    if t >= max_t {
                        sim.stop();
                        update.is_done = true;
                        tx.send(update.clone()).unwrap();
                        break;
                    }
                }
                {
                    let state = state.clone();
                    let state = state.lock();
                    let mut state = state.unwrap();
                    state.replace(sim.state.clone());
                }
                return sim.state.clone();
            });

            self.thread = Some(thread);
            
            if self.options.join {
                match self.thread.take() {
                    Some(thread) => {
                        let state = thread.join().unwrap();
                        self.state.clone().lock().unwrap().replace(state);
                    }
                    None => {
                        println!("SimRunner thread not found");
                    }
                }
            }
        } else {
            info!("[SimRunner] Running Simulation (unthreaded)...");
            let start_time = Instant::now();
            
            loop {
               
                t += dt;
                sim.step(t, dt);

                update.tick = update.tick + 1;
                update.state_sample = Some(sim.state.clone_without_logs());

                tx.send(update.clone()).unwrap();
               // let new_df = sim.state.get_df("UAV Test".to_string());
                // Append DF vertically

              //  dfs.push(new_df.lazy());
              

                if t >= max_t {
                    info!("[SimRunner] Simulation reached max_t");
                    sim.stop();
                
                    update.is_done = true;
                    tx.send(update.clone()).unwrap();
                    break;
                }
            }
            self.state.clone().lock().unwrap().replace(sim.state.clone());
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            info!("[SimRunner] Simulation Complete in {:?} ", duration);

            //info!("[SimRunner] Concatenating {:?} DataFrames", dfs.len());
            let start_time = Instant::now();
           
            //df = concat(dfs.as_slice(), UnionArgs { parallel: true, rechunk: false, to_supertypes: false }).unwrap().collect().unwrap();
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            info!("[SimRunner] Saved DataFrames Complete in {:?} ", duration);
          
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
