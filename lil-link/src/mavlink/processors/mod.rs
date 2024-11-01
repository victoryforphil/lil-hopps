use victory_data_store::database::DataView;

use super::core::MavlinkMessageType;

pub mod proc_attitude;
pub mod proc_command_ack;
pub mod proc_ekf_health;
pub mod proc_heartbeat;
pub mod proc_local_position;
pub mod proc_param_value;
pub mod proc_status_text;
pub mod proc_sys_status;

pub trait MavlinkMessageProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error>;
}

pub struct MavlinkGenericProcessor;

impl MavlinkMessageProcessor for MavlinkGenericProcessor {
    fn on_mavlink_message(
        msg: MavlinkMessageType,
        data_view: &mut DataView,
    ) -> Result<(), anyhow::Error> {
        match msg {
            MavlinkMessageType::HEARTBEAT(_) => {
                proc_heartbeat::HeartbeatProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::PARAM_VALUE(_) => {
                proc_param_value::ParamValueProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::STATUSTEXT(_) => {
                proc_status_text::StatusTextProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::SYS_STATUS(_) => {
                proc_sys_status::SysStatusProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::COMMAND_ACK(_) => {
                proc_command_ack::CommandAckProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::ATTITUDE(_) => {
                proc_attitude::AttitudeProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::LOCAL_POSITION_NED(_) => {
                proc_local_position::LocalPositionProcessor::on_mavlink_message(msg, data_view)
            }
            MavlinkMessageType::EKF_STATUS_REPORT(_) => {
                proc_ekf_health::EkfHealthProcessor::on_mavlink_message(msg, data_view)
            }

            _ => Ok(()),
        }
    }
}
