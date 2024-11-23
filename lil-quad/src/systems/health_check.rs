use std::collections::BTreeSet;

use lil_link::common::{
    identifiers::{IDENT_BASE_LOG, IDENT_BASE_STATUS, IDENT_STATUS_EKF, IDENT_STATUS_HEALTH},
    types::{ekf_status::QuadEkfStatus, health_status::QuadHealthStatus},
};
use log::info;
use serde::{Deserialize, Serialize};
use victory_broker::{broker::time::BrokerTime, task::{config::BrokerTaskConfig, subscription::BrokerTaskSubscription, trigger::BrokerTaskTrigger, BrokerTask}};
use victory_data_store::{database::view::DataView, topics::TopicKey};
use victory_wtf::Timespan;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheckConfig {
    pub check_ekf: Option<bool>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            check_ekf: Some(true),
        }
    }
}

pub struct HealthCheck {
    pub config: HealthCheckConfig,
}

impl HealthCheck {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new(HealthCheckConfig::default())
    }
}

impl BrokerTask for HealthCheck {
    fn init(&mut self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn get_config(&self) -> BrokerTaskConfig {
        BrokerTaskConfig::new("health_check")
            .with_trigger(BrokerTaskTrigger::Always)
            .with_subscription(BrokerTaskSubscription::new_latest(
                &TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_EKF)),
            ))
            .with_subscription(BrokerTaskSubscription::new_latest(&QuadHealthStatus::get_topic_key()))
    }

    fn on_execute(&mut self, inputs: &DataView, timing: &BrokerTime) -> Result<DataView, anyhow::Error> {
        let ekf_status: QuadEkfStatus = inputs
            .get_latest(&TopicKey::from_str(&format!(
                "{}/{}",
                IDENT_BASE_STATUS, IDENT_STATUS_EKF
            )))
            .unwrap_or_else(|_| QuadEkfStatus::new_null());

        let mut health_status = match inputs.get_latest(&QuadHealthStatus::get_topic_key()) {
            Ok(status) => status,
            Err(_) => QuadHealthStatus::new(false, None),
        };

        let mut out = DataView::new_timed(timing.time_monotonic.clone());

        match (self.config.check_ekf, ekf_status.is_healthy()) {
            (Some(true), Err(msg)) => {
                // Log if changed
                if health_status.reason != Some(msg.clone()) {
                    info!("EKF is changed to unhealthy: {}", msg);
                }
                health_status.healthy = false;
                health_status.reason = Some(msg);
            }
            _ => {
                // Log if changed
                if health_status.reason != None && health_status.healthy != true {
                    info!("EKF is changed to healthy");
                }
                health_status.healthy = true;
                health_status.reason = None;
            }
        }
        out.add_latest(&QuadHealthStatus::get_topic_key(), health_status)?;
        Ok(out)
    }
}
