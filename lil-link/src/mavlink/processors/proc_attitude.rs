use victory_data_store::{database::DataView, topics::TopicKey};

use crate::{
    common::{
        identifiers::{IDENT_ATTITUDE, IDENT_BASE_POSE},
        types::attitude::QuadAttitude,
    },
    mavlink::core::MavlinkMessageType,
};

use super::MavlinkMessageProcessor;

pub struct AttitudeProcessor;

impl MavlinkMessageProcessor for AttitudeProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let attitude_msg = match msg {
            MavlinkMessageType::ATTITUDE(attitude) => attitude,
            _ => return Err(anyhow::anyhow!("Expected attitude message, got {:?}", msg)),
        };

        let attitude =
            QuadAttitude::new_f32(attitude_msg.roll, attitude_msg.pitch, attitude_msg.yaw);
        let topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_POSE, IDENT_ATTITUDE));
        data_view.add_latest(&topic_key, attitude)?;
        Ok(())
    }
}
