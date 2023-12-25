// Headless Run
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
use lil_hopps::*;
use log::{LevelFilter, info};
use polars::io::{csv::CsvWriter, SerWriter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use simulation::{runner::SimRunner, runner_options::SimRunnerOptions};


fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("logs/lilhopps-main.log").unwrap(),
        ),
    ])
    .unwrap();
    let mut runner: SimRunner = SimRunner::new(SimRunnerOptions{
        max_t:20.0,
        dt: 0.001,
        threaded: true,
        join: false,
        send_every: 100,
    });

    runner.start();

    
    loop {
        let mut recv = runner.channel_rx.recv();

        if recv.is_err() {
            break;
        }

        let update = recv.unwrap();
        info!("Got update: {}, is done= {}",update.time.clone(),update.is_done.clone());
        if update.is_done {
            break;
        }
        
    }
    let state = runner.state.lock().unwrap();
    let state = state.to_owned().unwrap();
    let mut log_size = 0;
    for val in state.logs.values() {
        log_size += val.len();
    }
    info!("Simulation Finished. Log Keys: {}, Total Logs: {}", state.logs.keys().len(), log_size);


    
    
}
