use std::fs::File;

use lil_hopps::{*, simulation::{runner_options::SimRunnerOptions, runner::SimRunner}};
use log::LevelFilter;
use rerun::{demo_util::grid, external::glam};
use simplelog::{WriteLogger, CombinedLogger, TermLogger, Config, ColorChoice, TerminalMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {

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
    let mut runner: SimRunner = SimRunner::new(SimRunnerOptions::new(30.0));

    runner.start();
 


    println!(" Sim Finished. {} KB size, {:?}",  runner.df.estimated_size() / 1024, runner.df);

    let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal").spawn()?;

    let pos_y =  runner.df.column("UAV Test.pose.position.y").unwrap().f32().unwrap().to_vec();
    let pos_x =  runner.df.column("UAV Test.pose.position.x").unwrap().f32().unwrap().to_vec();
    let pos_z =  runner.df.column("UAV Test.pose.position.z").unwrap().f32().unwrap().to_vec();
    
    let mut points = Vec::new();
    for i in 0..pos_x.len() {
        let (x,y,z) = (pos_x[i].unwrap(), pos_y[i].unwrap(), pos_z[i].unwrap());
        points.push(glam::Vec3::new(x, y, z));
        rec.set_time_sequence("frame_idx", i as i64 );
        rec.set_time_seconds("sensor_time", i as f64 * runner.options.dt); 
    
        rec.log("uav/location",  
            &rerun::Boxes3D::from_centers_and_half_sizes(
                [(x,y,z)], 
                [(0.1, 0.1, 0.1)]))?;

        // Draw floor
        rec.log("floor", &rerun::Boxes3D::from_centers_and_half_sizes(
            [glam::Vec3::new(0.0, 0.0, 0.0)], 
            [glam::Vec3::new(10.0, 10.0, 0.1)]))?;
    }

    

    Ok(())
}
