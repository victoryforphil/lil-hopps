use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};



use super::{Simulation, SimulationState, runner_options::SimRunnerOptions};

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


impl SimRunner {
    pub fn new(options: SimRunnerOptions) -> Self {
        let (tx, rx) = channel();
        Self {
            channel_tx: tx,
            channel_rx: rx,
            thread: None,
            options
        }
    }

    pub fn start(&mut self) {
        let tx = self.channel_tx.clone();
        let mut sim = Simulation::new();
        let max_t = self.options.max_t;
        let dt = self.options.dt;
        let mut t= 0.0;
        sim.init();
        if self.options.threaded{
            let thread = thread::spawn(move || loop {
                t += dt;
                sim.step(t, dt);
                tx.send(sim.state.clone()).unwrap();
                println!("SimRunner tick: t={:?}", t);
                if t >= max_t {
                    sim.stop();
                    tx.send(sim.state.clone()).unwrap();
                    break;
                }
    
            });
            self.thread = Some(thread);

            if self.options.join{
               match self.thread.take(){
                   Some(thread) => {
                       thread.join().unwrap();
                   },
                   None => {
                       println!("SimRunner thread not found");
                   }
               }
            }
        }else{
            loop {
                t += dt;
                sim.step(t, dt);
                tx.send(sim.state.clone()).unwrap();
                println!("SimRunner tick: t={:?}", t);
                if t >= max_t {
                    sim.stop();
                    tx.send(sim.state.clone()).unwrap();
                    break;
                }
            }
        }
    }
}
