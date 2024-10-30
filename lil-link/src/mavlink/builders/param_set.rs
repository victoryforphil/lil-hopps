use crate::common::types::parameter::QuadParameter;

pub fn mavlink_build_param_set_message(
    param_req: QuadParameter,
) -> Option<mavlink::ardupilotmega::MavMessage> {
    if param_req.ack {
        return None;
    }

    let param_cmd =
        mavlink::ardupilotmega::MavMessage::PARAM_SET(mavlink::ardupilotmega::PARAM_SET_DATA {
            param_value: param_req.value as f32,
            target_system: 1,
            target_component: 1,
            param_id: param_req.byte_id(),
            ..Default::default()
        });
    Some(param_cmd)
}
