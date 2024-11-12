use std::collections::BTreeSet;

use lil_link::common::types::request_takeoff::QuadTakeoffRequest;
use log::info;
use victory_commander::system::System;
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::Timepoint;

pub struct TimedTakeoff {
    pub takeoff_time: Timepoint,
    pub height: f32,
    current_time: Timepoint,
    sent: bool,
}

impl TimedTakeoff {
    pub fn new(takeoff_time: Timepoint, height: f32) -> Self {
        Self {
            takeoff_time,
            height,
            current_time: Timepoint::zero(),
            sent: false,
        }
    }
}

impl System for TimedTakeoff {
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
        _inputs: &victory_data_store::database::view::DataView,
        dt: victory_wtf::Timespan,
    ) -> victory_data_store::database::view::DataView {
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.takeoff_time || self.sent {
            return out;
        }
        info!(
            "Sending takeoff command as time {} has elapsed",
            self.current_time.secs()
        );
        let takeoff_msg = QuadTakeoffRequest {
            height: self.height,
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/takeoff"), takeoff_msg)
            .expect("Failed to add takeoff message");
        out
    }

    fn cleanup(&mut self) {}

    fn name(&self) -> String {
        "timed_takeoff".to_string()
    }
}
