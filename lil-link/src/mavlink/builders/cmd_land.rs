use mavlink::ardupilotmega::COMMAND_LONG_DATA;

use crate::common::types::request_land::QuadLandRequest;

pub fn mavlink_build_cmd_land_message(land_request: QuadLandRequest) -> Option<mavlink::ardupilotmega::MavMessage> {
    if land_request.ack {
        return None;
    }
    let land_cmd = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {         
        command: mavlink::ardupilotmega::MavCmd::MAV_CMD_NAV_LAND,
        ..Default::default()
    });
    Some(land_cmd)
}
