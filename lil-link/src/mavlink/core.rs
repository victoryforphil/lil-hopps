use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam_channel::{Receiver, Sender};
use mavlink::{
    ardupilotmega::MavMessage,
    error::MessageReadError,
    MavConnection,
};
use tracing::{debug, info, warn};

use crate::mavlink::helpers::MavLinkHelper;

#[derive(thiserror::Error, Debug)]
pub enum QuadLinkError {
    #[error("Mavlink error: {0}")]
    MavlinkError(mavlink::error::MessageReadError),
    #[error("Channel recv error: {0}")]
    ChannelRecvError(crossbeam_channel::RecvError),
    #[error("Channel send error: {0}")]
    ChannelSendError(crossbeam_channel::SendError<MavlinkMessageType>),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Generic error: {0}")]
    GenericError(String),
    #[error("No Pending Data")]
    NoData,
}
pub type MavlinkMessageType = mavlink::ardupilotmega::MavMessage;
pub struct QuadLinkCore {
    recv_channels: (Sender<MavlinkMessageType>, Receiver<MavlinkMessageType>),
    transmit_channels: (Sender<MavlinkMessageType>, Receiver<MavlinkMessageType>),
    connection_string: String,
    recv_thread: Option<thread::JoinHandle<()>>,
    send_thread: Option<thread::JoinHandle<()>>,
    heartbeat_thread: Option<thread::JoinHandle<()>>,
}

pub type QuadlinkCoreHandle = Arc<Mutex<QuadLinkCore>>;

impl QuadLinkCore {
    pub fn new(connection_string: &str) -> Result<Self, anyhow::Error> {
        let (recv_tx, recv_rx): (Sender<_>, Receiver<_>) = crossbeam_channel::bounded(500);
        let (transmit_tx, transmit_rx): (Sender<_>, Receiver<_>) = crossbeam_channel::bounded(500);

        Ok(Self {
            recv_channels: (recv_tx, recv_rx),
            transmit_channels: (transmit_tx, transmit_rx),
            connection_string: connection_string.to_string(),
            recv_thread: None,
            send_thread: None,
            heartbeat_thread: None,
        })
    }

    pub fn start_thread(&mut self) -> Result<(), QuadLinkError> {
        let con_string: String = self.connection_string.clone();
        let recv_channels = self.recv_channels.clone();
        let transmit_channels = self.transmit_channels.clone();
        thread::spawn(move || {
            Self::start_thread_inner(con_string, recv_channels, transmit_channels).unwrap();
        });
        Ok(())
    }
    fn start_thread_inner(
        con_string: String,
        recv_channels: (Sender<MavlinkMessageType>, Receiver<MavlinkMessageType>),
        transmit_channels: (Sender<MavlinkMessageType>, Receiver<MavlinkMessageType>),
    ) -> Result<(), QuadLinkError> {
        // 1. Make the connection

        info!(
            "Quadlink / MAVLink => Connecting to MAVLink with connection string: {}",
            con_string
        );
        let mut mav_con: Box<dyn MavConnection<MavMessage> + Send + Sync> =
            mavlink::connect::<MavMessage>(&con_string)
                .map_err(|e| QuadLinkError::ConnectionError(e.to_string()))
                .unwrap();

        info!("Quadlink / MAVLink => Sending inital settings");

        mav_con.set_protocol_version(mavlink::MavlinkVersion::V2);

        mav_con
            .send(
                &mavlink::MavHeader::default(),
                &MavLinkHelper::request_parameters(),
            )
            .unwrap();
        mav_con
            .send(
                &mavlink::MavHeader::default(),
                &MavLinkHelper::request_stream(),
            )
            .unwrap();
        let mav_con = Arc::new(mav_con);

        info!("Quadlink / MAVLink => Starting main threads...");
        debug!("Quadlink / MAVLink => Spawning heartbeat thread");
        // Heartbeat thread
        thread::spawn({
            let vehicle = mav_con.clone();
            move || loop {
                let res: Result<usize, mavlink::error::MessageWriteError> =
                    vehicle.send_default(&MavLinkHelper::heartbeat_message());
                if res.is_ok() {
                    debug!("Quadlink / MAVLink => Heartbeat sent");
                    thread::sleep(Duration::from_secs(1));
                } else {
                    warn!("Quadlink / MAVLink => Heartbeat send failed: {:?}", res);
                }
            }
        });

        // Send thread
        debug!("Quadlink / MAVLink => Spawning send thread");
        let vehicle = mav_con.clone();
        thread::spawn(move || loop {
            let (_, rx) = &transmit_channels;
            if let Ok(msg) = rx.recv() {
                vehicle.send(&mavlink::MavHeader::default(), &msg).unwrap();
            }
        });

        debug!("Quadlink / MAVLink => Spawning recv thread");
        // Recv thread
        thread::spawn(move || {
            let mav_con = mav_con.clone();
            loop {
                match mav_con.recv() {
                    Ok((_header, msg)) => {
                        let (recv_tx, _) = recv_channels.clone();
                        recv_tx.send(msg).unwrap();
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
        Ok(())
    }

    pub fn send(&self, msg: &MavlinkMessageType) -> Result<(), QuadLinkError> {
        let (tx, _) = &self.transmit_channels;
        tx.send(msg.clone())
            .map_err(QuadLinkError::ChannelSendError)
    }

    pub fn recv(&self) -> Result<Vec<MavlinkMessageType>, QuadLinkError> {
        let mut data = Vec::new();
        let (_tx, rx) = &self.recv_channels;
        while let Ok(msg) = rx.try_recv() {
            data.push(msg);
        }
        Ok(data)
    }
}
