use log::info;
use victory_data_store::{database::view::DataView, topics::TopicKey};

use crate::{
    common::identifiers::{IDENT_BASE_LOG, IDENT_STATUS_TEXT},
    mavlink::core::MavlinkMessageType,
};

use super::MavlinkMessageProcessor;

pub struct StatusTextProcessor;

impl MavlinkMessageProcessor for StatusTextProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        let status_text_msg = match msg {
            MavlinkMessageType::STATUSTEXT(status_text) => status_text,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected status text message, got {:?}",
                    msg
                ))
            }
        };

        let text = status_text_msg
            .text
            .iter()
            .map(|c| *c as char)
            .collect::<String>();
        let text = text.trim_end_matches(char::from(0));
        info!("UAV status text: \n\t{}\n --------------", text);

        let topic_key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_LOG, IDENT_STATUS_TEXT));
        data_view.add_latest(&topic_key, text)?;

        Ok(())
    }
}
