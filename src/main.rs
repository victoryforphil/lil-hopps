use std::{thread, time::Duration};

use simulation::runner::SimRunner;

pub mod types;
pub mod uav;
pub mod simulation;

fn main() {
    let mut runner = SimRunner::new();
    runner.start();

    while(runner.channel_rx.recv().unwrap().running){
        println!("SimRunner received state");
        thread::sleep(Duration::from_millis(100));
    }
}
