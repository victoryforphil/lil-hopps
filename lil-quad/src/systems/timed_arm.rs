use std::time::Duration;

use log::info;
use serde::{Deserialize, Serialize};
use victory_broker::task::{config::BrokerTaskConfig, trigger::BrokerTaskTrigger, BrokerTask};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::{Timepoint, Timespan};

pub struct TimedArm {
    arm_time: Timepoint,
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

impl BrokerTask for TimedArm {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        self.current_time = Timepoint::now();
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("timed_arm")
            .with_trigger(BrokerTaskTrigger::Always)
    }

    fn on_execute(&mut self, _inputs: &DataView) -> Result<DataView, anyhow::Error> {
        let dt = Timespan::from_duration(Duration::from_millis(100));
        self.current_time = self.current_time.clone() + dt;
        let mut out = DataView::new();

        if self.current_time <= self.arm_time || self.sent {
            return Ok(out);
        }
        info!("Arming as time {} has elapsed", self.current_time.secs());
        let arm_msg = ArmMessage {
            arm: true,
            ack: false,
        };
        self.sent = true;
        out.add_latest(&TopicKey::from_str("cmd/arm"), arm_msg)?;
        Ok(out)
    }
}
