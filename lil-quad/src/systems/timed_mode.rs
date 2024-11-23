use std::time::Duration;

use lil_link::common::types::{mode::QuadMode, request_arm::QuadSetModeRequest};
use log::info;
use victory_broker::{broker::time::BrokerTime, task::{config::BrokerTaskConfig, trigger::BrokerTaskTrigger, BrokerTask}};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

pub struct TimedMode {
    mode_time: Timepoint,
    mode: QuadMode,
    sent: bool,
}

impl TimedMode {
    pub fn new(mode_time: Timepoint, mode: QuadMode) -> Self {
        Self {
            mode_time,
            mode,
            sent: false,
        }
    }
}

impl BrokerTask for TimedMode {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("timed_mode")
            .with_trigger(BrokerTaskTrigger::Always)
    }

    fn on_execute(&mut self, _inputs: &DataView, timing: &BrokerTime) -> Result<DataView, anyhow::Error> {
        let mut out = DataView::new_timed(timing.time_monotonic.clone());

        if timing.time_monotonic <= self.mode_time || self.sent {
            return Ok(out);
        }
        info!(
            "Setting mode as time {} has elapsed",
            timing.time_monotonic.secs()
        );
        let mode_msg = QuadSetModeRequest {
            mode: self.mode.clone(),
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/mode"), mode_msg)?;
        Ok(out)
    }
}
