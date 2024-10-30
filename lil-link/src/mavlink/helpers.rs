use crate::common::types::{
    autopilot_status::QuadAutopilotStatus, mode::QuadMode, sensor_status::QuadSensorStatus,
};

use super::ardu_modes::ArduMode;

pub struct MavLinkHelper;

impl MavLinkHelper {
    /// Create a heartbeat message using 'ardupilotmega' dialect
    pub fn heartbeat_message() -> mavlink::ardupilotmega::MavMessage {
        mavlink::ardupilotmega::MavMessage::HEARTBEAT(mavlink::ardupilotmega::HEARTBEAT_DATA {
            custom_mode: 0,
            mavtype: mavlink::ardupilotmega::MavType::MAV_TYPE_QUADROTOR,
            autopilot: mavlink::ardupilotmega::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
            base_mode: mavlink::ardupilotmega::MavModeFlag::empty(),
            system_status: mavlink::ardupilotmega::MavState::MAV_STATE_STANDBY,
            mavlink_version: 0x3,
        })
    }

    /// Create a message requesting the parameters list
    pub fn request_parameters() -> mavlink::ardupilotmega::MavMessage {
        mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_LIST(
            mavlink::ardupilotmega::PARAM_REQUEST_LIST_DATA {
                target_system: 0,
                target_component: 0,
            },
        )
    }

    /// Create a message enabling data streaming
    pub fn request_stream() -> mavlink::ardupilotmega::MavMessage {
        mavlink::ardupilotmega::MavMessage::REQUEST_DATA_STREAM(
            mavlink::ardupilotmega::REQUEST_DATA_STREAM_DATA {
                target_system: 0,
                target_component: 0,
                req_stream_id: 0,
                req_message_rate: 50,
                start_stop: 1,
            },
        )
    }

    /// Decode MavModeFlag to QuadModeStatus
    pub fn decode_mode_flag(flag: mavlink::ardupilotmega::MavModeFlag) -> QuadAutopilotStatus {
        QuadAutopilotStatus {
            custom_mode_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED),
            test_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_TEST_ENABLED),
            auto_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_AUTO_ENABLED),
            guided_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_GUIDED_ENABLED),
            stabilize_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_STABILIZE_ENABLED),
            hil_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_HIL_ENABLED),
            manual_input_enabled: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_MANUAL_INPUT_ENABLED),
            safety_armed: flag
                .contains(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED),
        }
    }

    pub fn decode_sensor_health(
        health: mavlink::ardupilotmega::MavSysStatusSensor,
    ) -> QuadSensorStatus {
        QuadSensorStatus {
            gyro: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO),
            accel: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL),
            mag: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG),
            abs_pressure: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE),
            diff_pressure: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE),
            gps: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_GPS),
            optical_flow: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW),
            vision_position: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_VISION_POSITION),
            laser_position: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_LASER_POSITION),
            external_ground_truth: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_EXTERNAL_GROUND_TRUTH),
            rate_control: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ANGULAR_RATE_CONTROL),
            attitude_stabilization: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ATTITUDE_STABILIZATION),
            yaw_position: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_YAW_POSITION),
            altitude_control: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_Z_ALTITUDE_CONTROL),
            xy_position_control: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_XY_POSITION_CONTROL),
            motor_control: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS),
            rc_receiver: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_RC_RECEIVER),
            gyro2: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO2),
            accel2: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL2),
            mag2: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG2),
            geofence: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_GEOFENCE),
            ahrs: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_AHRS),
            terrain: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_TERRAIN),
            reverse_motor: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_REVERSE_MOTOR),
            logging: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_LOGGING),
            battery: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_BATTERY),
            proximity: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_PROXIMITY),
            satcom: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_SATCOM),
            prearm_check: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_PREARM_CHECK),
            obstacle_avoidance: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_OBSTACLE_AVOIDANCE),
            propulsion: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_PROPULSION),
            extension: health.contains(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_EXTENSION_USED),
        }
    }
    pub fn quad_mode_to_mav_mode(mode: &QuadMode) -> ArduMode {
        match mode {
            QuadMode::Stabilize => ArduMode::Stabilize,
            QuadMode::Acro => ArduMode::Acro,
            QuadMode::AltHold => ArduMode::AltHold,
            QuadMode::Auto => ArduMode::Auto,
            QuadMode::Guided => ArduMode::Guided,
            QuadMode::Loiter => ArduMode::Loiter,
            QuadMode::Return => ArduMode::RTL,
            QuadMode::Land => ArduMode::Land,
            QuadMode::PosHold => ArduMode::PosHold,
            QuadMode::Brake => ArduMode::Brake,
            QuadMode::Follow => ArduMode::Follow,
        }
    }
}
