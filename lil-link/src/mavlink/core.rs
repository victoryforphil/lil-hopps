use core::error;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam_channel::{Receiver, Sender};
use mavlink::{
    ardupilotmega::{MavMessage, MavModeFlag, COMMAND_LONG_DATA, SET_MODE_DATA},
    error::MessageReadError,
    MavConnection,
};
use tracing::{debug, info, warn};

use crate::mavlink::{helpers::MavLinkHelper, types::QuadNED};

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

    fn process_send_message(
        quad_msg: &QuadMessageTx,
        mav_con: Arc<Box<dyn MavConnection<MavMessage> + Send + Sync>>,
    ) {
        match quad_msg {
            QuadMessageTx::SetArm(arm) => {
                let arm_value = if *arm { 1.0 } else { 0.0 };
                let arm_cmd = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(
                    mavlink::ardupilotmega::COMMAND_LONG_DATA {
                        param1: arm_value,
                        param2: 21196.,
                        param3: 0.0,
                        param4: 0.0,
                        param5: 0.0,
                        param6: 0.0,
                        param7: 0.0,
                        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
                        target_system: 0,
                        target_component: 0,
                        confirmation: 0,
                    },
                );
                info!("Sending Ardupilot Arm Command: {:#?}", arm_cmd);
                mav_con
                    .send(&mavlink::MavHeader::default(), &arm_cmd)
                    .unwrap();
            }
            QuadMessageTx::SetMode(mode) => {
                // See: https://ardupilot.org/dev/docs/mavlink-get-set-flightmode.html
                let mav_mode = MavLinkHelper::quad_mode_to_mav_mode(mode);
                let mode_cmd =
                    mavlink::ardupilotmega::MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
                        param1: MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED.bits() as f32,
                        param2: mav_mode.to_u32() as f32,
                        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_DO_SET_MODE,
                        ..Default::default()
                    });
                info!(
                    "Quadlink / MAVLink => Sending MAVLink Mode Command: {:#?}",
                    mode_cmd
                );
                mav_con
                    .send(&mavlink::MavHeader::default(), &mode_cmd)
                    .unwrap();
            }
            QuadMessageTx::ParamSet(_, _) => todo!(),
            QuadMessageTx::TakeOff(height) => {
                let takeoff_cmd =
                    mavlink::ardupilotmega::MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
                        param3: 5.0,
                        param7: *height,
                        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_NAV_TAKEOFF,
                        ..Default::default()
                    });
                info!(
                    "Quadlink / MAVLink => Sending MAVLink Takeoff Command: {:#?}",
                    takeoff_cmd
                );
                mav_con
                    .send(&mavlink::MavHeader::default(), &takeoff_cmd)
                    .unwrap();
            }
        }
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
        recv_channels: (Sender<QuadMessageRx>, Receiver<QuadMessageRx>),
        transmit_channels: (Sender<QuadMessageTx>, Receiver<QuadMessageTx>),
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
                Self::process_send_message(&msg, vehicle.clone());
            }
        });

        debug!("Quadlink / MAVLink => Spawning recv thread");
        // Recv thread
        thread::spawn(move || {
            let mav_con = mav_con.clone();
            loop {
                match mav_con.recv() {
                    Ok((_header, msg)) => {
                        match msg {
                            mavlink::ardupilotmega::MavMessage::HEARTBEAT(hb) => {
                                let system_status = format!("{:?}", hb.system_status);
                                let mode_status = MavLinkHelper::decode_mode_flag(hb.base_mode);
                                let (recv_tx, _) = recv_channels.clone();
                                recv_tx
                                    .send(QuadMessageRx::ModeStatus(mode_status))
                                    .unwrap();
                                recv_tx
                                    .send(QuadMessageRx::SimpleStatus(system_status))
                                    .unwrap();
                            }
                            // Parameter
                            mavlink::ardupilotmega::MavMessage::PARAM_VALUE(pv) => {
                                let param_id = pv.param_id;
                                // Covert to name from byte array of chars
                                let param_name =
                                    param_id.iter().map(|c| *c as char).collect::<String>();
                                let param_name = param_name.trim_end_matches(char::from(0));
                                let value = pv.param_value as f64;
                                let (recv_tx, _) = recv_channels.clone();
                                recv_tx
                                    .send(QuadMessageRx::ParamValue(
                                        param_name.to_ascii_lowercase(),
                                        value,
                                    ))
                                    .unwrap();
                            }
                            mavlink::ardupilotmega::MavMessage::STATUSTEXT(st) => {
                                // Convert to string
                                let text = st.text.iter().map(|c| *c as char).collect::<String>();
                                let text = text.trim_end_matches(char::from(0));
                                info!(" ---\n\t MAV Link Status Text: {:?}", text);
                            }

                            //SYS_STATUS
                            mavlink::ardupilotmega::MavMessage::SYS_STATUS(ss) => {
                                let sesnor_health = ss.onboard_control_sensors_health;
                                let sensor_status =
                                    MavLinkHelper::decode_sensor_health(sesnor_health);
                            }
                            mavlink::ardupilotmega::MavMessage::LOCAL_POSITION_NED(lp) => {
                                let (recv_tx, _) = recv_channels.clone();
                                recv_tx
                                    .send(QuadMessageRx::Position(QuadNED {
                                        x: lp.x as f64,
                                        y: lp.y as f64,
                                        z: lp.z as f64,
                                    }))
                                    .unwrap();
                            }
                            mavlink::ardupilotmega::MavMessage::ATTITUDE(a) => {
                                //info!("Attitude: {:?}", a);
                                let (recv_tx, _) = recv_channels.clone();
                                recv_tx
                                    .send(QuadMessageRx::Attitude(
                                        a.roll as f64,
                                        a.pitch as f64,
                                        a.yaw as f64,
                                    ))
                                    .unwrap();
                            }
                            mavlink::ardupilotmega::MavMessage::COMMAND_ACK(ca) => {
                                debug!("Command Ack: {:?}", ca);
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
        Ok(())
    }

    pub fn send(&self, msg: &QuadMessageTx) -> Result<(), QuadLinkError> {
        let (tx, _) = &self.transmit_channels;
        tx.send(msg.clone())
            .map_err(|e| QuadLinkError::ChannelSendError(e))
    }

    pub fn recv(&self) -> Result<Vec<QuadMessageRx>, QuadLinkError> {
        let mut data = Vec::new();
        let (_tx, rx) = &self.recv_channels;
        while let Ok(msg) = rx.try_recv() {
            data.push(msg);
        }
        Ok(data)
    }
}
