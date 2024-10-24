use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use log::info;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_wtf::Timespan;

use crate::mavlink::{
    core::{QuadLinkCore, QuadlinkCoreHandle},
    messages::{QuadMessageRx, QuadMessageTx},
    types::{QuadArmRequest, QuadAttitudeTemp, QuadSetModeRequest, QuadTakeoffRequest},
};

pub struct QuadlinkSystem {
    mavlink: QuadlinkCoreHandle,
}

impl QuadlinkSystem {
    pub fn new(mavlink: QuadlinkCoreHandle) -> Self {
        Self { mavlink }
    }

    pub fn new_from_connection_string(connection_string: &str) -> Result<Self, anyhow::Error> {
        let mavlink = QuadLinkCore::new(connection_string)?;
        Ok(Self {
            mavlink: Arc::new(Mutex::new(mavlink)),
        })
    }

    fn proccess_message(&mut self, msg: QuadMessageRx, data_view: &mut DataView) {
        match msg {
            QuadMessageRx::ParamValue(p_key, p_value) => {
                let topic_key = TopicKey::from_str(&format!("params/{}", p_key));
                data_view
                    .add_latest(&topic_key, p_value)
                    .expect("Failed to add latest parameter value");
            }
            QuadMessageRx::Attitude(roll, pitch, yaw) => {
                let topic_key = TopicKey::from_str("attitude");
                data_view
                    .add_latest(&topic_key, QuadAttitudeTemp { roll, pitch, yaw })
                    .expect("Failed to add latest attitude");
            }

            QuadMessageRx::ModeStatus(mode_status) => {
                let topic_key = TopicKey::from_str("status/mode");
                data_view
                    .add_latest(&topic_key, mode_status)
                    .expect("Failed to add latest mode status");
            }
            QuadMessageRx::SimpleStatus(status) => {
                let topic_key = TopicKey::from_str("status/simple");
                data_view
                    .add_latest(&topic_key, status)
                    .expect("Failed to add latest status");
            }
            QuadMessageRx::Position(quad_ned) => {
                let topic_key = TopicKey::from_str("position/ned");
                data_view
                    .add_latest(&topic_key, quad_ned)
                    .expect("Failed to add latest position");
            }
        };
    }
}

impl System for QuadlinkSystem {
    fn init(&mut self) {
        let mut mavlink = self.mavlink.lock().unwrap();
        mavlink.start_thread().unwrap();
    }

    fn get_subscribed_topics(&self) -> std::collections::BTreeSet<TopicKey> {
        let mut topics = BTreeSet::new();
        topics.insert(TopicKey::from_str("cmd/arm"));
        topics.insert(TopicKey::from_str("cmd/mode"));
        topics.insert(TopicKey::from_str("cmd/takeoff"));
        topics
    }

    fn execute<'a>(&mut self, inputs: &'a DataView, _: Timespan) -> DataView {
        let mut output = DataView::new();

        let mut msgs = vec![];
        {
            let mavlink = self.mavlink.lock().unwrap();
            msgs = mavlink.recv().unwrap();
        }
        for msg in msgs {
            self.proccess_message(msg, &mut output);
        }

        let arm_req: Result<QuadArmRequest, _> = inputs.get_latest(&TopicKey::from_str("cmd/arm"));
        let mode_req: Result<QuadSetModeRequest, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/mode"));
        
        match arm_req {
            Ok(arm_req) => {
                if !arm_req.ack {
                    let arm_msg = QuadMessageTx::SetArm(arm_req.arm);
                    {
                        let mavlink = self.mavlink.lock().unwrap();
                        info!("QuadLink received arm request from cmd/arm: {:?}", arm_req);
                        mavlink.send(&arm_msg).unwrap();
                    }
                    let mut new_ack = arm_req;
                    new_ack.ack = true;
                    output
                        .add_latest(&TopicKey::from_str("cmd/arm"), new_ack)
                        .expect("Failed to add latest arm ack");
                }
            }
            Err(_) => {}
        }
        if let Ok(mode_req) = mode_req {
            if !mode_req.ack {
                let mode_msg = QuadMessageTx::SetMode(mode_req.mode.clone());
                {
                    let mavlink = self.mavlink.lock().unwrap();
                    info!("QuadLink received mode request from cmd/mode: {:?}", mode_req);
                    mavlink.send(&mode_msg).unwrap();
                }
                output
                    .add_latest(&TopicKey::from_str("cmd/mode/ack"), true)
                    .expect("Failed to add latest mode ack");
            }
           
            
        }
        
        let takeoff_req: Result<QuadTakeoffRequest, _> = inputs.get_latest(&TopicKey::from_str("cmd/takeoff"));
        if let Ok(takeoff_req) = takeoff_req {
            let takeoff_msg = QuadMessageTx::TakeOff(takeoff_req.height);
            if !takeoff_req.ack {
                {
                    let mavlink = self.mavlink.lock().unwrap();
                    mavlink.send(&takeoff_msg).unwrap();
                }
                output
                    .add_latest(&TopicKey::from_str("cmd/takeoff/ack"), true)
                    .expect("Failed to add latest takeoff ack");
            }
        }
        output
    }

    fn cleanup(&mut self) {}
}
