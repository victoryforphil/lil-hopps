use mavlink::ardupilotmega::{MavCmd, MavMessage, COMMAND_LONG_DATA, LED_CONTROL_DATA};

pub fn mavlink_build_cmd_led_message(
    red: u8,
    green: u8, 
    blue: u8,
) -> Option<mavlink::ardupilotmega::MavMessage> {
    let mut bytes: Vec<u8> = vec![red, green, blue];
    // Fill til 24
    bytes.resize(24, 0);

    let led_cmd = mavlink::ardupilotmega::MavMessage::LED_CONTROL(
        LED_CONTROL_DATA {
            instance: 0,
            pattern: 0,
            custom_len: 3,
            custom_bytes: bytes.try_into().unwrap(),
            target_system: 0,
            target_component: 0,
        },
    );
    Some(led_cmd)
}
