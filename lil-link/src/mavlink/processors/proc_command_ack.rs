use log::info;
use victory_data_store::{database::view::DataView, topics::TopicKey};

use crate::{
    common::identifiers::{IDENT_BASE_LOG, IDENT_COMMAND_ACK},
    mavlink::core::MavlinkMessageType,
};

use super::MavlinkMessageProcessor;

pub struct CommandAckProcessor;

impl MavlinkMessageProcessor for CommandAckProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let command_ack_msg = match msg {
            MavlinkMessageType::COMMAND_ACK(command_ack) => command_ack,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected command ack message, got {:?}",
                    msg
                ))
            }
        };

        info!(
            "QUAD COMMAND ACK:\n\t {:?} -> {:?} \n  --------------",
            command_ack_msg.command, command_ack_msg.result
        );

        let topic_key = TopicKey::from_str(
            &format!(
                "{}/{}/{:?}",
                IDENT_BASE_LOG, IDENT_COMMAND_ACK, command_ack_msg.command
            )
            .to_ascii_lowercase(),
        );
        let result = format!("{:?}", command_ack_msg.result);
        data_view.add_latest(&topic_key, result)?;

        Ok(())
    }
}
