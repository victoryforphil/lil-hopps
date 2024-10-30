use victory_data_store::{database::DataView, topics::TopicKey};

use crate::{
    common::identifiers::{IDENT_BASE_STATUS, IDENT_STATUS_SENSORS},
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
        Ok(())
    }
}
