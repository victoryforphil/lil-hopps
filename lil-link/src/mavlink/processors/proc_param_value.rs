use victory_data_store::topics::TopicKey;

use crate::{common::identifiers::IDENT_BASE_PARAMS, mavlink::core::MavlinkMessageType};

use super::MavlinkMessageProcessor;

pub struct ParamValueProcessor;

impl MavlinkMessageProcessor for ParamValueProcessor {
    fn on_mavlink_message(
        msg: crate::mavlink::core::MavlinkMessageType,
        data_view: &mut victory_data_store::database::DataView,
    ) -> Result<(), anyhow::Error> {
        let param_value_msg = match msg {
            MavlinkMessageType::PARAM_VALUE(param_value) => param_value,
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected param value message, got {:?}",
                    msg
                ))
            }
        };

        let param_id = param_value_msg.param_id;

        // Convert from byte array to string
        let param_name = param_id.iter().map(|c| *c as char).collect::<String>();
        let param_name = param_name
            .trim_end_matches(char::from(0))
            .to_ascii_lowercase();
        let value = param_value_msg.param_value as f64;

        // Write to database
        let key = TopicKey::from_str(&format!("{}/{}", IDENT_BASE_PARAMS, param_name));
        data_view.add_latest(&key, value)?;

        Ok(())
    }
}
