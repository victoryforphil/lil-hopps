use log::trace;
use victory_data_store::{database::DataView, topics::TopicKey};

use crate::{
    common::{
        identifiers::{IDENT_STATUS_MODE, IDENT_STATUS_SYSTEM},
        types::autopilot_status::QuadAutopilotStatus,
    },
    mavlink::{core::MavlinkMessageType, helpers::MavLinkHelper},
};

use super::MavlinkMessageProcessor;

pub struct HeartbeatProcessor;

impl MavlinkMessageProcessor for HeartbeatProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        trace!("Processing heartbeat message");
        let heartbeat_msg = match msg {
            MavlinkMessageType::HEARTBEAT(heartbeat) => heartbeat,
            _ => return Err(anyhow::anyhow!("Expected heartbeat message, got {:?}", msg)),
        };

        let system_status: String = format!("{:?}", heartbeat_msg.system_status);
        let mode_status: QuadAutopilotStatus =
            MavLinkHelper::decode_mode_flag(heartbeat_msg.base_mode);

        // Write to database
        data_view.add_latest(&TopicKey::from_str(IDENT_STATUS_MODE), mode_status)?;
        data_view.add_latest(&TopicKey::from_str(IDENT_STATUS_SYSTEM), system_status)?;

        Ok(())
    }
}
