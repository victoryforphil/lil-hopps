use victory_data_store::{database::DataView, topics::TopicKey};

use crate::{
    common::identifiers::*,
    mavlink::{core::MavlinkMessageType, helpers::MavLinkHelper},
};

use super::MavlinkMessageProcessor;

pub struct SysStatusProcessor;

impl MavlinkMessageProcessor for SysStatusProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let sys_status_msg = match msg {
            MavlinkMessageType::SYS_STATUS(sys_status) => sys_status,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected sys status message, got {:?}",
                    msg
                ))
            }
        };
        let sesnor_health = sys_status_msg.onboard_control_sensors_health;
        let sensor_status = MavLinkHelper::decode_sensor_health(sesnor_health);

        let topic_key =
            TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_SENSORS));
        data_view.add_latest(&topic_key, sensor_status)?;

        let battery_status = sys_status_msg.battery_remaining;
        let drop_rate_comm = sys_status_msg.drop_rate_comm;
        let voltage = sys_status_msg.voltage_battery;
        let current = sys_status_msg.current_battery;
        let comm_errors = sys_status_msg.errors_comm;
        let errors_count = sys_status_msg.errors_count1 + sys_status_msg.errors_count2 + sys_status_msg.errors_count3; 

        let battery_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_BATTERY));
        data_view.add_latest(&battery_topic_key, battery_status)?;

        let drop_rate_comm_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_DROP_RATE_COMM));
        data_view.add_latest(&drop_rate_comm_topic_key, drop_rate_comm)?;

        let voltage_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_VOLTAGE));
        data_view.add_latest(&voltage_topic_key, voltage)?;

        let current_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_CURRENT));
        data_view.add_latest(&current_topic_key, current)?;

        let comm_errors_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_COMM_ERRORS));
        data_view.add_latest(&comm_errors_topic_key, comm_errors)?;

        let errors_count_topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_ERRORS_COUNT));
        data_view.add_latest(&errors_count_topic_key, errors_count)?;

        Ok(())
    }
}
