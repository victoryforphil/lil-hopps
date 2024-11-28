use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use log::{debug, info};
use victory_broker::{broker::time::BrokerTime, task::{config::BrokerTaskConfig, subscription::BrokerTaskSubscription, trigger::BrokerTaskTrigger, BrokerTask}};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::Timespan;

use crate::common::types::{
    pose_ned::QuadPoseNED, request_arm::QuadSetModeRequest, request_land::QuadLandRequest, request_led::QuadLedRequest, request_mode_set::QuadArmRequest, request_takeoff::QuadTakeoffRequest
};

use super::{
    builders::{
        cmd_arm::mavlink_build_arm_message, cmd_land::mavlink_build_cmd_land_message, cmd_led::mavlink_build_cmd_led_message, cmd_mode::mavlink_build_mode_message, cmd_takeoff::mavlink_build_cmd_takeoff_message, cmd_waypoint::mavlink_build_cmd_waypoint_message
    },
    core::{QuadLinkCore, QuadlinkCoreHandle},
    processors::{MavlinkGenericProcessor, MavlinkMessageProcessor},
};

pub struct QuadlinkSystem {
    mavlink: QuadlinkCoreHandle,

    last_requested_waypoint: Option<QuadPoseNED>,
}

impl QuadlinkSystem {
    pub fn new(mavlink: QuadlinkCoreHandle) -> Self {
        Self {
            mavlink,
            last_requested_waypoint: None,
        }
    }

    pub fn new_from_connection_string(connection_string: &str) -> Result<Self, anyhow::Error> {
        let mavlink = QuadLinkCore::new(connection_string)?;
        Ok(Self {
            mavlink: Arc::new(Mutex::new(mavlink)),
            last_requested_waypoint: None,
        })
    }
}

impl BrokerTask for QuadlinkSystem{

    fn init(&mut self) -> Result<(), anyhow::Error> {
        info!("QuadlinkSystem // Initializing");
        let mut mavlink = self.mavlink.lock().unwrap();
        mavlink.start_thread()?;
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("quadlink-mavlink")
            .with_trigger(BrokerTaskTrigger::Always)
            .with_subscription(BrokerTaskSubscription::new_updates_only(
                &TopicKey::from_str("cmd")
            ))
            // cmd/waypoint is new_latest
            .with_subscription(BrokerTaskSubscription::new_latest(
                &TopicKey::from_str("cmd/waypoint")
            ))
            .with_subscription(BrokerTaskSubscription::new_latest(
                &TopicKey::from_str("cmd/led")
            ))
    }

    fn on_execute(&mut self, inputs: &DataView, timing: &BrokerTime) -> Result<DataView, anyhow::Error> {
        let mut output = DataView::new_timed(timing.time_monotonic.clone());

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
        let arm_topic = TopicKey::from_str("cmd/arm");
        let mode_topic = TopicKey::from_str("cmd/mode");
        let arm_req: Result<QuadArmRequest, _> = inputs.get_latest(&arm_topic);
        let mode_req: Result<QuadSetModeRequest, _> = inputs.get_latest(&mode_topic);

        if let Ok(arm_req) = arm_req {
           if !arm_req.ack {
                match mavlink_build_arm_message(arm_req.clone()) {
                    Some(arm_msg) => {
                        let mavlink = self.mavlink.lock().unwrap();
                        info!("QuadLink received arm request from cmd/arm: {:?}", arm_req);
                        mavlink.send(&arm_msg).unwrap();
                        let mut new_ack = arm_req;
                        new_ack.ack();
                        output
                            .add_latest(&arm_topic, new_ack)
                            .expect("Failend to add latest arm ack");
                    }
                    None => {}
                }
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
                    .add_latest(&mode_topic, new_ack)
                    .expect("Failed to add latest mode ack");
            }
        }

        let takeoff_req: Result<QuadTakeoffRequest, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/takeoff"));
        if let Ok(takeoff_req) = takeoff_req {
            if !takeoff_req.ack {
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
        }

        let land_req: Result<QuadLandRequest, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/land"));
        if let Ok(land_req) = land_req {
            if !land_req.ack {
                match mavlink_build_cmd_land_message(land_req.clone()) {
                    Some(land_msg) => {
                        let mavlink = self.mavlink.lock().unwrap();
                        info!(
                            "QuadLink received land request from cmd/land: {:?}",
                            land_req.clone()
                        );
                        mavlink.send(&land_msg).unwrap();
                    }
                    None => {}
                }
                let mut new_ack = land_req;
                new_ack.ack();
                output
                    .add_latest(&new_ack.get_topic_key(), new_ack)
                    .expect("Failed to add latest land ack");
            }
        }

        let waypoint_req: Result<QuadPoseNED, _> =
            inputs.get_latest(&TopicKey::from_str("cmd/waypoint"));

        match (waypoint_req, self.last_requested_waypoint.clone()) {
            // If the waypoint has changed, send the new waypoint
            (Ok(waypoint_req), Some(last_waypoint))
                if waypoint_req.distance(&last_waypoint) > 0.1 =>
            {
                self.last_requested_waypoint = Some(waypoint_req.clone());
                info!("Quadlink // System // Sending UPDATED waypoint: {:?}", waypoint_req);
                match mavlink_build_cmd_waypoint_message(waypoint_req.clone()) {
                    Some(waypoint_msg) => {
                        let mavlink = self.mavlink.lock().unwrap();
                        mavlink.send(&waypoint_msg).unwrap();
                    }
                    None => {}
                }
            }
            // If no previous waypoint, set and send
            (Ok(waypoint_req), None) => {
                self.last_requested_waypoint = Some(waypoint_req.clone());
                info!("Quadlink // System // Sending NEW waypoint: {:?}", waypoint_req);
                match mavlink_build_cmd_waypoint_message(waypoint_req.clone()) {
                    Some(waypoint_msg) => {
                        let mavlink = self.mavlink.lock().unwrap();
                        mavlink.send(&waypoint_msg).unwrap();
                    }
                    None => {}
                }
            }
            _ => {
         
            }
        }

        let led_topic = TopicKey::from_str("cmd/led");
        // Add LED control
        let led_req: Result<QuadLedRequest, _> =
            inputs.get_latest(&led_topic);
        if let Ok(led_req) = led_req {
            if !led_req.ack {
                match mavlink_build_cmd_led_message(led_req.red, led_req.green, led_req.blue) {
                    Some(led_msg) => {
                        info!("Quadlink // System // Sending LED control: {:?}", led_req);
                        let mavlink = self.mavlink.lock().unwrap();
                        mavlink.send(&led_msg).unwrap();
                    }
                    None => {}
                }
                let mut new_ack = led_req;
                new_ack.ack();
                output
                    .add_latest(&led_topic, new_ack)
                    .expect("Failed to add latest led ack");
            }
        }

        Ok(output)
    }
    
}
