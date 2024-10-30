use mavlink::ardupilotmega::{MavModeFlag, COMMAND_LONG_DATA};

use crate::{common::types::request_arm::QuadSetModeRequest, mavlink::helpers::MavLinkHelper};

pub fn mavlink_build_mode_message(
    mode_req: QuadSetModeRequest,
) -> Option<mavlink::ardupilotmega::MavMessage> {
    if mode_req.ack {
        return None;
    }
    // See: https://ardupilot.org/dev/docs/mavlink-get-set-flightmode.html
    let mav_mode = MavLinkHelper::quad_mode_to_mav_mode(&mode_req.mode);
    let mode_cmd = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
        param1: MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED.bits() as f32,
        param2: mav_mode.to_u32() as f32,
        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_DO_SET_MODE,
        ..Default::default()
    });
    Some(mode_cmd)
}
