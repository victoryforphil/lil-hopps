use std::{collections::HashMap, sync::Arc};

use lil_broker::Timestamp;
use lil_sym::{DefaultScenario, RerunDataview, SimRunnerConfig, SimThreadedRunner};
use tracing::{error, info};

fn main() {
    env_logger::init();
    let scenario = DefaultScenario {};

    let mut sim_config = SimRunnerConfig::default();
    sim_config.max_t = Timestamp::from_seconds(10.0);

    let mut sim_runner = SimThreadedRunner::new(Arc::new(scenario), sim_config);

    let channels = sim_runner.start();

    let mut db = HashMap::new();

    if let Ok(sim_updates_rx) = channels {
        let mut sim_update = sim_updates_rx.recv();

        while sim_update.is_ok() {
            let update = sim_update.unwrap();
            info!("Simulation update: {:?}", update.state.t.seconds());
            db = update.state.uav_dbs;
            sim_update = sim_updates_rx.recv();
            // Wait 100ms
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        info!("Simulation completed");
    } else {
        error!("Error starting simulation")
    }
    // Sleep for 5s to allow the logger to flush
    info!("Sleeping for 5s to allow logger to flush");
    std::thread::sleep(std::time::Duration::from_secs(5));
    for (uav_id, uav_db) in db.iter() {
        // Print DB

        let mut rerun = RerunDataview::new(
            format!("uav-{uav_id}"),
            "lil-sym/main".to_string(),
            "manual".to_string(),
            uav_db.clone(),
        );
        rerun.logging_start();
        rerun.update().unwrap();
    }
}
