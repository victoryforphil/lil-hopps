use mavlink::ardupilotmega::COMMAND_LONG_DATA;

use crate::common::types::request_takeoff::QuadTakeoffRequest;

pub fn mavlink_build_cmd_takeoff_message(
    takeoff_request: QuadTakeoffRequest,
) -> Option<mavlink::ardupilotmega::MavMessage> {
    if takeoff_request.ack {
        return None;
    }
    let height = takeoff_request.height;
    let takeoff_cmd = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
        param3: 5.0,
        param7: height,
        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_NAV_TAKEOFF,
        ..Default::default()
    });
    Some(takeoff_cmd)
}
