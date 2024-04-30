use lil_broker::Timestamp;
use lil_quad::{runner::{UAVRunner, UAVRunnerCommand, UAVRunnerConfig, UAVRunnerStatus, UAVThreadedRunner}, uav::{MockUAVRuntime, UAVRuntime, UAV}};
use tracing::info;


use tracing_subscriber::layer::SubscriberExt;



fn main() {
    env_logger::init();
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default())
    ).expect("setup tracy layer");
    let uav_runtime = MockUAVRuntime::new().as_arc_mutex();
    let uav = UAV::new(uav_runtime);
    let config = UAVRunnerConfig::default().set_wait();
    let mut runner = UAVThreadedRunner::new(uav,config.clone());
    
    let client_channels = runner.start().unwrap();

    client_channels
        .command_channel
        .send(UAVRunnerCommand::Start(Timestamp::from_seconds(100.0)))
        .unwrap();
    
    let mut state = client_channels.state_channel.recv().unwrap();

    while state.state != UAVRunnerStatus::Completed {
        let recv = client_channels.state_channel.recv();
        match recv {
            Ok(s) => {
                state = s;
               // info!("State: {:?}", state);
            }
            Err(e) => {
                info!("Error: {:?}", e);
                break;
            }
        }
     
    }

   
}
