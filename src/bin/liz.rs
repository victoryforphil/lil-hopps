// Headless Run
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
use lil_hopps::*;
use log::LevelFilter;
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
    let mut runner: SimRunner = SimRunner::new(SimRunnerOptions::new(100.0));

    runner.start();
 
    println!(" Sim Finished. {} KB size, {:?}",  runner.df.estimated_size() / 1024, runner.df);
    

    // Save the dataframe to a csv file
 
    let mut file = std::fs::File::create("logs/simout_liz.csv").unwrap();
    // Print si
    CsvWriter::new(&mut file).finish(&mut  runner.df).unwrap();
}
