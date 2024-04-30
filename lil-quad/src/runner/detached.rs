use std::{sync::{Arc, Mutex}, thread::JoinHandle};

use crate::uav::{UAVRuntime, UAV};

use super::{UAVRunner, UAVRunnerClientChannels, UAVRunnerConfig};



pub struct UAVThreadedRunner{
    pub uav: UAV,
    config: UAVRunnerConfig,
    thread: Option<JoinHandle<()>>,
}


impl UAVThreadedRunner{
    pub fn new (uav: UAV, config: UAVRunnerConfig) -> Self{
        Self{
            config,
            uav,
            thread: None,
        }
    }

    pub fn start(&mut self) -> Result<UAVRunnerClientChannels, anyhow::Error>{
        let uav = self.uav.clone();
        let config = self.config.clone();
        let mut uav_runner = UAVRunner::new(config, uav);
        let client_channels =  uav_runner.channels.get_client_channels();
        let thread = std::thread::spawn(move || {
            
            uav_runner.init();
         
            uav_runner.start();

             
        });

        self.thread = Some(thread);
        Ok(client_channels)
    }
}