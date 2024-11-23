use std::time::Duration;

use lil_link::common::types::{mode::QuadMode, request_arm::QuadSetModeRequest};
use log::info;
use victory_broker::task::{config::BrokerTaskConfig, trigger::BrokerTaskTrigger, BrokerTask};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

pub struct TimedMode {
    mode_time: Timepoint,
    mode: QuadMode,
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

impl BrokerTask for TimedMode {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        self.current_time = Timepoint::now();
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("timed_mode")
            .with_trigger(BrokerTaskTrigger::Always)
    }

    fn on_execute(&mut self, _inputs: &DataView) -> Result<DataView, anyhow::Error> {
        let dt = Timespan::from_duration(Duration::from_millis(100));
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.mode_time || self.sent {
            return Ok(out);
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
        out.add_latest(&TopicKey::from_str("cmd/mode"), mode_msg)?;
        Ok(out)
    }
}
