use victory_data_store::{database::view::DataView, topics::TopicKey};

use crate::{
    common::identifiers::{IDENT_BASE_STATUS, IDENT_STATUS_EKF, IDENT_STATUS_SENSORS},
    mavlink::{core::MavlinkMessageType, helpers::MavLinkHelper},
};

use super::MavlinkMessageProcessor;

pub struct EkfHealthProcessor;

impl MavlinkMessageProcessor for EkfHealthProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let ekf_status_msg = match msg {
            MavlinkMessageType::EKF_STATUS_REPORT(ekf_status) => ekf_status,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected EKF status message, got {:?}",
                    msg
                ))
            }
        };
        let ekf_health: mavlink::ardupilotmega::EkfStatusFlags = ekf_status_msg.flags;
        let ekf_status = MavLinkHelper::decode_ekf_status(ekf_health);
        let topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_STATUS, IDENT_STATUS_EKF));
        data_view.add_latest(&topic_key, ekf_status)?;
        Ok(())
    }
}
