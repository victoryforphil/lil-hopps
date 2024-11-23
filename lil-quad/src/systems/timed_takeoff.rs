use std::time::Duration;

use lil_link::common::types::request_takeoff::QuadTakeoffRequest;
use log::info;
use victory_broker::{broker::time::BrokerTime, task::{config::BrokerTaskConfig, trigger::BrokerTaskTrigger, BrokerTask}};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

pub struct TimedTakeoff {
    takeoff_time: Timepoint,
    height: f32,
    sent: bool,
}

impl TimedTakeoff {
    pub fn new(takeoff_time: Timepoint, height: f32) -> Self {
        Self {
            takeoff_time,
            height,
            sent: false,
        }
    }
}

impl BrokerTask for TimedTakeoff {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("timed_takeoff")
            .with_trigger(BrokerTaskTrigger::Always)
    }

    fn on_execute(&mut self, _inputs: &DataView, timing: &BrokerTime) -> Result<DataView, anyhow::Error> {
        let mut out = DataView::new_timed(timing.time_monotonic.clone());

        if timing.time_monotonic <= self.takeoff_time || self.sent {
            return Ok(out);
        }
        info!(
            "Sending takeoff command as time {} has elapsed",
            timing.time_monotonic.secs()
        );
        let takeoff_msg = QuadTakeoffRequest {
            height: self.height,
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/takeoff"), takeoff_msg)?;
        Ok(out)
    }
}
