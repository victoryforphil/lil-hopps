use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use super::{Simulation, SimulationState};

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
}

impl SimRunner {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            channel_tx: tx,
            channel_rx: rx,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let tx = self.channel_tx.clone();
        let mut sim = Simulation::new();
        let start_time = std::time::Instant::now();
        let max_duration = Duration::from_secs(3);
        sim.init();
        let thread = thread::spawn(move || loop {
            let dt: Duration = start_time.elapsed();
            let t = Instant::now();
            sim.step(t, dt);
            tx.send(sim.state.clone()).unwrap();
            thread::sleep(Duration::from_millis(10));
            println!("SimRunner tick: {:?}", dt);
            if dt > max_duration {
                sim.stop();
                tx.send(sim.state.clone()).unwrap();
                break;
            }

        });
        self.thread = Some(thread);
    }
}
