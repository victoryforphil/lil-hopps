use std::{collections::HashMap, sync::Arc};

use lil_broker::Timestamp;
use lil_sym::{DefaultScenario, SimRunnerConfig, SimThreadedRunner};
use tracing::{error, info};

fn main() {
    env_logger::init();
    let mut scenario = DefaultScenario {};

    let mut sim_config = SimRunnerConfig::default();
    sim_config.max_t = Timestamp::from_seconds(100.0);
    let mut sim_runner = SimThreadedRunner::new(Arc::new(scenario), sim_config);
    let channels = sim_runner.start();

    let mut db = HashMap::new();

    if let Ok(sim_updates_rx) = channels {
        let mut sim_update  = sim_updates_rx.recv();

        while sim_update.is_ok() {
            let update = sim_update.unwrap();
            db = update.state.uav_dbs;
            sim_update = sim_updates_rx.recv();
        }



      info!("Simulation completed");

        for (id, db) in db.iter() {
            info!("UAV Data base: {}", id);
            let mut db = db.lock().unwrap();
            
            let value = db.query_get_latest(vec!["".to_string()].into()).unwrap();
            info!("Latest value: {:?}", value);
        }
         
    }else{
        error!("Error starting simulation")
    }

}
