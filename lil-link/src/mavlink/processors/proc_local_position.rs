use victory_data_store::{database::DataView, topics::TopicKey};

use crate::{
    common::{
        identifiers::{IDENT_BASE_POSE, IDENT_POSE_NED},
        types::{pose_ned::QuadPoseNED, vector3::Vector3},
    },
    mavlink::core::MavlinkMessageType,
};

use super::MavlinkMessageProcessor;

pub struct LocalPositionProcessor;

impl MavlinkMessageProcessor for LocalPositionProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let local_position_msg = match msg {
            MavlinkMessageType::LOCAL_POSITION_NED(local_position) => local_position,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected local position message, got {:?}",
                    msg
                ))
            }
        };

        let quad_pose = QuadPoseNED::new_position_and_velocity(
            Vector3::new_f32(
                local_position_msg.x,
                local_position_msg.y,
                local_position_msg.z,
            ),
            Vector3::new_f32(
                local_position_msg.vx,
                local_position_msg.vy,
                local_position_msg.vz,
            ),
        );

        let topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_POSE, IDENT_POSE_NED));
        data_view.add_latest(&topic_key, quad_pose)?;
        Ok(())
    }
}
