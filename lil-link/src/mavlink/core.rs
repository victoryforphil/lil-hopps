use core::error;
use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crossbeam_channel::{Receiver, Sender};
use log::info;
use mavlink::{ardupilotmega::MavMessage, error::MessageReadError, MavConnection};

use crate::mavlink::helpers::MavLinkHelper;

use super::messages::{QuadMessageRx, QuadMessageTx};

#[derive(thiserror::Error, Debug)]
pub enum QuadLinkError {
    #[error("Mavlink error: {0}")]
    MavlinkError(mavlink::error::MessageReadError),
    #[error("Channel recv error: {0}")]
    ChannelRecvError(crossbeam_channel::RecvError),
    #[error("Channel send error: {0}")]
    ChannelSendError(crossbeam_channel::SendError<QuadMessageTx>),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Generic error: {0}")]
    GenericError(String),
    #[error("No Pending Data")]
    NoData,
}

pub struct QuadLinkCore {
    recv_channels: (Sender<QuadMessageRx>, Receiver<QuadMessageRx>),
    transmit_channels: (Sender<QuadMessageTx>, Receiver<QuadMessageTx>),
    mav_con: Option<Arc<Box<dyn MavConnection<MavMessage>>>>,
    connection_string: String,
    thread: Option<thread::JoinHandle<()>>,
}

impl QuadLinkCore {
    pub fn new(connection_string: &str) -> Result<Self, anyhow::Error> {
        let (recv_tx, recv_rx): (Sender<_>, Receiver<_>) = crossbeam_channel::bounded(500);
        let (transmit_tx, transmit_rx): (Sender<_>, Receiver<_>) = crossbeam_channel::bounded(500);

        Ok(Self {
            recv_channels: (recv_tx, recv_rx),
            transmit_channels: (transmit_tx, transmit_rx),
            mav_con:None,
            connection_string: connection_string.to_string(),
            thread: None,
        })
    }

    pub fn start_thread(&mut self) -> Result<(), QuadLinkError>{

        let con_string: String = self.connection_string.clone();
        let recv_channels = self.recv_channels.clone();
        let thread = std::thread::spawn( move || {
            info!("Staring Mavlink Thread");
            info!("Mavlink Connection String: {}", con_string);
            let mut mav_con = mavlink::connect::<MavMessage>(&con_string)
            .map_err(|e| QuadLinkError::ConnectionError(e.to_string())).unwrap();
            
            info!("Sending inital settings");

            mav_con.set_protocol_version(mavlink::MavlinkVersion::V2);
            
            mav_con
                .send(&mavlink::MavHeader::default(), &MavLinkHelper::request_parameters())
                .unwrap();
            mav_con
                .send(&mavlink::MavHeader::default(), &MavLinkHelper::request_stream())
                .unwrap();
            let mav_con = Arc::new(mav_con);

            // Heartbeat thread
            thread::spawn({
                let vehicle = mav_con.clone();
                move || loop {
                    let res = vehicle.send_default(&MavLinkHelper::heartbeat_message());
                    if res.is_ok() {
                        thread::sleep(Duration::from_secs(1));
                    } else {
                        println!("send failed: {res:?}");
                    }
                }
            });

            loop{
                match mav_con.clone().recv() {
                    Ok((_header, msg)) => {
                    
                        match msg {
                            
                            mavlink::ardupilotmega::MavMessage::HEARTBEAT(hb) => {
                                
                            }
                            // Parameter
                            mavlink::ardupilotmega::MavMessage::PARAM_VALUE(pv) => {
                                let param_id = pv.param_id;
                                // Covert to name from byte array of chars
                                let param_name = param_id.iter().map(|c| *c as char).collect::<String>();
                                let value = pv.param_value as f64;
                                let (recv_tx, _) = recv_channels.clone();
                                recv_tx.send(QuadMessageRx::ParamValue(param_name, value)).unwrap();
                            }
                            _ => {}
                        }
                        //println!("received: {msg:?}");
                    }
                    Err(MessageReadError::Io(e)) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            //no messages currently available to receive -- wait a while
                            thread::sleep(Duration::from_secs(1));
                           
                        } else {
                            println!("recv error: {e:?}");
                           
                        }
                    }
                    // messages that didn't get through due to parser errors are ignored
                    _ => {}
                }
            }
        });
        self.thread = Some(thread);
        Ok(())
    }


    pub fn send(&self, msg: &QuadMessageTx)-> Result<(), QuadLinkError>{
        let (tx, _) = &self.transmit_channels;
        tx.send(msg.clone()).map_err(|e| QuadLinkError::ChannelSendError(e))
    }

    pub fn recv(&self)-> Result<Vec<QuadMessageRx>, QuadLinkError>{
        let mut data = Vec::new();
        let (_tx, rx) = &self.recv_channels;
        while let Ok(msg) = rx.try_recv(){
            data.push(msg);
        }
        Ok(data)
    }
    
}
