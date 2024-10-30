use crate::common::types::request_mode_set::QuadArmRequest;

pub fn mavlink_build_arm_message(
    arm_req: QuadArmRequest,
) -> Option<mavlink::ardupilotmega::MavMessage> {
    if arm_req.ack {
        return None;
    }

    let arm_value = if arm_req.arm { 1.0 } else { 0.0 };
    let arm_cmd = mavlink::ardupilotmega::MavMessage::COMMAND_LONG(
        mavlink::ardupilotmega::COMMAND_LONG_DATA {
            param1: arm_value,
            param2: 21196., // 21196 is the code for arm/disarm forcefully
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
            command: mavlink::ardupilotmega::MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
            target_system: 0,
            target_component: 0,
            confirmation: 0,
        },
    );
    Some(arm_cmd)
}
