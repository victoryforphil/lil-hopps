use mavlink::ardupilotmega::{PositionTargetTypemask, COMMAND_LONG_DATA, SET_POSITION_TARGET_LOCAL_NED_DATA};

use crate::common::types::{pose_ned::QuadPoseNED, request_land::QuadLandRequest};

pub fn mavlink_build_cmd_waypoint_message(desired_pose: QuadPoseNED) -> Option<mavlink::ardupilotmega::MavMessage> {
    let land_cmd = mavlink::ardupilotmega::MavMessage::SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA {
        x: desired_pose.position.x as f32,
        y: desired_pose.position.y as f32,
        z: desired_pose.position.z as f32,
        coordinate_frame: mavlink::ardupilotmega::MavFrame::MAV_FRAME_LOCAL_NED,
        type_mask: PositionTargetTypemask::from_bits(0b110111111000).unwrap(),
        ..Default::default()
    });
    Some(land_cmd)
}
