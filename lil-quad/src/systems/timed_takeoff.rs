use std::time::Duration;

use lil_link::common::types::request_takeoff::QuadTakeoffRequest;
use log::info;
use victory_broker::task::{config::BrokerTaskConfig, trigger::BrokerTaskTrigger, BrokerTask};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

pub struct TimedTakeoff {
    takeoff_time: Timepoint,
    height: f32,
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

impl BrokerTask for TimedTakeoff {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        self.current_time = Timepoint::now();
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("timed_takeoff")
            .with_trigger(BrokerTaskTrigger::Always)
    }

    fn on_execute(&mut self, _inputs: &DataView) -> Result<DataView, anyhow::Error> {
        let dt = Timespan::from_duration(Duration::from_millis(100)); // Assuming 100ms rate
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.takeoff_time || self.sent {
            return Ok(out);
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
        out.add_latest(&TopicKey::from_str("cmd/takeoff"), takeoff_msg)?;
        Ok(out)
    }
}
