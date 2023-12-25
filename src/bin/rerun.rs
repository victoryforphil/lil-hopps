// Headless Run
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
use lil_hopps::{*, logging::LogData};
use log::{LevelFilter, info};
use polars::io::{csv::CsvWriter, SerWriter};
use rerun::{Position3D, components::Scalar, Rotation3D, Quaternion, Position2D};
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
        dt: 0.01,
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

    let rec = rerun::RecordingStreamBuilder::new("lil-sim").spawn().unwrap();
    for (key, val) in state.logs.iter() {
   
        let first = val.first().unwrap();
        
        for entry in val.iter() {
           
            rec.set_time_seconds("sim_time", entry.time);
            match &entry.data{
                LogData::Pose(pose) => {
                    let point = rerun::Points3D::new(
                        [Position3D::new(pose.position.x, pose.position.y, pose.position.z)],
                    );
                    rec.log(entry.key.clone(), &point);

                    // Log each component of the pose
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "scalar", "x"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(pose.position.x as f64)),
                    );
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "scalar", "y"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(pose.position.y as f64)),
                    );
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "scalar", "z"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(pose.position.z as f64)),
                    );
                    
                   

                },
                LogData::Movement(movement) => {
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_velocity", "x"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_vel.x as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_velocity", "y"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_vel.y as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_velocity", "z"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_vel.z as f64)),
                    ).unwrap();

                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_velocity", "x"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_vel.x as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_velocity", "y"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_vel.y as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_velocity", "z"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_vel.z as f64)),
                    ).unwrap();

                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_acceleration", "x"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_accel.x as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_acceleration", "y"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_accel.y as f64)),
                    ).unwrap();
                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "linear_acceleration", "z"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.lin_accel.z as f64)),
                    ).unwrap();

                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_acceleration", "x"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_accel.x as f64)),
                    ).unwrap();

                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_acceleration", "y"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_accel.y as f64)),
                    ).unwrap();

                    rec.log(
                        format!("{}/{}/{}", entry.key.clone(), "angular_acceleration", "z"),
                        &rerun::TimeSeriesScalar::new(Scalar::from(movement.ang_accel.z as f64)),
                    ).unwrap();

        
                    
                },
                LogData::Motor(motor) => {
                    rec.log(format!("{}/scalar/value", entry.key.clone()), &rerun::TimeSeriesScalar::new(Scalar::from(motor.current_value as f64)));
                    rec.log(format!("{}/scalar/offset", entry.key.clone()), &rerun::Points2D::new([Position2D::new(motor.motor_offset_b.x, motor.motor_offset_b.y), Position2D::new(0.0, 0.0)]).with_radii([0.5, 1.0])).unwrap();
                    rec.log(format!("{}/scalar/force", entry.key.clone()), &rerun::TimeSeriesScalar::new(Scalar::from(motor.motor_force_n as f64)));
                }
                LogData::String(_) => {},
                LogData::Float(value) => {
                    rec.log(entry.key.clone(), &rerun::TimeSeriesScalar::new(Scalar::from(value.clone() as f64)));
                },
            }
        }
    }
    
}
