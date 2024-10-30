use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use log::info;
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_wtf::Timespan;

use crate::common::types::{
    request_arm::QuadSetModeRequest, request_mode_set::QuadArmRequest,
    request_takeoff::QuadTakeoffRequest,
};

use super::{
    builders::{
        cmd_arm::mavlink_build_arm_message, cmd_mode::mavlink_build_mode_message,
        cmd_takeoff::mavlink_build_cmd_takeoff_message,
    },
    core::{QuadLinkCore, QuadlinkCoreHandle},
    processors::{MavlinkGenericProcessor, MavlinkMessageProcessor},
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

    fn execute(&mut self, inputs: &DataView, _: Timespan) -> DataView {
        let mut output = DataView::new();
        #[allow(unused_variables)]
        let mut msgs = vec![];
        {
            let mavlink = self.mavlink.lock().unwrap();
            msgs = mavlink.recv().unwrap();
        }
        // Read incoming MavLink messages
        for msg in msgs {
            MavlinkGenericProcessor::on_mavlink_message(msg, &mut output).unwrap();
        }

        let arm_req: Result<QuadArmRequest, _> = inputs.get_latest(&TopicKey::from_str("cmd/arm"));
        let mode_req: Result<QuadSetModeRequest, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/mode"));

        if let Ok(arm_req) = arm_req {
            match mavlink_build_arm_message(arm_req.clone()) {
                Some(arm_msg) => {
                    let mavlink = self.mavlink.lock().unwrap();
                    info!("QuadLink received arm request from cmd/arm: {:?}", arm_req);
                    mavlink.send(&arm_msg).unwrap();
                    let mut new_ack = arm_req;
                    new_ack.ack();
                    output
                        .add_latest(&new_ack.get_topic_key(), new_ack)
                        .expect("Failend to add latest arm ack");
                }
                None => {}
            }
        }
        if let Ok(mode_req) = mode_req {
            if !mode_req.ack {
                match mavlink_build_mode_message(mode_req.clone()) {
                    Some(mode_msg) => {
                        let mavlink = self.mavlink.lock().unwrap();
                        info!(
                            "QuadLink received mode request from cmd/mode: {:?}",
                            mode_req.clone()
                        );
                        mavlink.send(&mode_msg).unwrap();
                    }
                    None => {}
                }
                let mut new_ack = mode_req;
                new_ack.ack();
                output
                    .add_latest(&new_ack.get_topic_key(), new_ack)
                    .expect("Failed to add latest mode ack");
            }
        }

        let takeoff_req: Result<QuadTakeoffRequest, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/takeoff"));
        if let Ok(takeoff_req) = takeoff_req {
            match mavlink_build_cmd_takeoff_message(takeoff_req.clone()) {
                Some(takeoff_msg) => {
                    let mavlink = self.mavlink.lock().unwrap();
                    info!(
                        "QuadLink received takeoff request from cmd/takeoff: {:?}",
                        takeoff_req.clone()
                    );
                    mavlink.send(&takeoff_msg).unwrap();
                }
                None => {}
            }
            let mut new_ack = takeoff_req;
            new_ack.ack();
            output
                .add_latest(&new_ack.get_topic_key(), new_ack)
                .expect("Failed to add latest takeoff ack");
        }
        output
    }

    fn cleanup(&mut self) {}
}
