use std::collections::BTreeSet;

use log::info;
use serde::{Deserialize, Serialize};
use victory_commander::system::System;
use victory_data_store::{database::DataView, topics::TopicKey};
use victory_wtf::Timepoint;

pub struct TimedArm {
    pub arm_time: Timepoint,
    current_time: Timepoint,
    sent: bool,
}

impl TimedArm {
    pub fn new(arm_time: Timepoint) -> Self {
        Self {
            arm_time,
            current_time: Timepoint::zero(),
            sent: false,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArmMessage {
    pub arm: bool,
    pub ack: bool,
}

impl System for TimedArm {
    fn init(&mut self) {
        self.current_time = Timepoint::zero();
    }

    fn get_subscribed_topics(
        &self,
    ) -> std::collections::BTreeSet<victory_data_store::topics::TopicKey> {
        BTreeSet::new()
    }

    fn execute(
        &mut self,
        inputs: &victory_data_store::database::DataView,
        dt: victory_wtf::Timespan,
    ) -> victory_data_store::database::DataView {
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.arm_time || self.sent {
            return out;
        }
        info!("Arming as time {} has elapsed", self.current_time.secs());
        let arm_msg = ArmMessage {
            arm: true,
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/arm"), arm_msg);
        out
    }

    fn cleanup(&mut self) {}

    fn name(&self) -> String {
        "timed_arm".to_string()
    }
}
