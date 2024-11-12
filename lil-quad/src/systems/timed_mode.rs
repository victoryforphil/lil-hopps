use std::collections::BTreeSet;

use lil_link::common::types::{mode::QuadMode, request_arm::QuadSetModeRequest};
use log::info;
use victory_commander::system::System;
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::Timepoint;

pub struct TimedMode {
    pub mode_time: Timepoint,
    pub mode: QuadMode,
    current_time: Timepoint,
    sent: bool,
}

impl TimedMode {
    pub fn new(mode_time: Timepoint, mode: QuadMode) -> Self {
        Self {
            mode_time,
            mode,
            current_time: Timepoint::zero(),
            sent: false,
        }
    }
}

impl System for TimedMode {
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
        _inputs: &DataView,
        dt: victory_wtf::Timespan,
    ) ->DataView {
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.mode_time || self.sent {
            return out;
        }
        info!(
            "Setting mode as time {} has elapsed",
            self.current_time.secs()
        );
        let mode_msg = QuadSetModeRequest {
            mode: self.mode.clone(),
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/mode"), mode_msg)
            .expect("Failed to add mode message");
        out
    }

    fn cleanup(&mut self) {}

    fn name(&self) -> String {
        "timed_mode".to_string()
    }
}
