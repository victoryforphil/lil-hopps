use mavlink::{error::MessageReadError, MavConnection};
use std::{env, sync::Arc, thread, time::Duration};
use lil_link::mavlink::helpers::MavLinkHelper;
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: mavlink-dump (tcpout|tcpin|udpout|udpin|udpbcast|serial|file):(ip|dev|path):(port|baud)"
        );
        return;
    }
    

    // It's possible to change the mavlink dialect to be used in the connect call
    let mut mavconn = mavlink::connect::<mavlink::ardupilotmega::MavMessage>(&args[1]).unwrap();

    // here as an example we force the protocol version to mavlink V1:
    // the default for this library is mavlink V2
    mavconn.set_protocol_version(mavlink::MavlinkVersion::V2);

    let vehicle = Arc::new(mavconn);
    vehicle
        .send(&mavlink::MavHeader::default(), &MavLinkHelper::request_parameters())
        .unwrap();
    vehicle
        .send(&mavlink::MavHeader::default(), &MavLinkHelper::request_stream())
        .unwrap();

    thread::spawn({
        let vehicle = vehicle.clone();
        move || loop {
            let res = vehicle.send_default(&MavLinkHelper::heartbeat_message());
            if res.is_ok() {
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("send failed: {res:?}");
            }
        }
    });

    loop {
        match vehicle.recv() {
            Ok((_header, msg)) => {
            
                match msg {
                    
                    mavlink::ardupilotmega::MavMessage::HEARTBEAT(hb) => {
                        println!("heartbeat: {hb:?}");
                    }
                    // Parameter
                    mavlink::ardupilotmega::MavMessage::PARAM_VALUE(pv) => {
                        let param_id = pv.param_id;
                        // Covert to name from byte array of chars
                        let param_name = param_id.iter().map(|c| *c as char).collect::<String>();
                        println!("param: {param_name} ");   
                    }
                    _ => {}
                }
                //println!("received: {msg:?}");
            }
            Err(MessageReadError::Io(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    //no messages currently available to receive -- wait a while
                    thread::sleep(Duration::from_secs(1));
                    continue;
                } else {
                    println!("recv error: {e:?}");
                    break;
                }
            }
            // messages that didn't get through due to parser errors are ignored
            _ => {}
        }
    }
}

