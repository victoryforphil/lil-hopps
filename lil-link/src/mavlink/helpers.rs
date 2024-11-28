use crate::common::types::{
    autopilot_status::QuadAutopilotStatus, ekf_status::QuadEkfStatus, mode::QuadMode,
    sensor_status::QuadSensorStatus,
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
                target_system: 1,
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
                req_message_rate: 30,
                start_stop: 1,
            },
        )
    }

    /// Decode MavModeFlag to QuadModeStatus
    pub fn decode_mode_flag(flag: mavlink::ardupilotmega::MavModeFlag) -> QuadAutopilotStatus {
        QuadAutopilotStatus {
            custom_mode_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED),
            test_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_TEST_ENABLED),
            auto_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_AUTO_ENABLED),
            guided_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_GUIDED_ENABLED),
            stabilize_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_STABILIZE_ENABLED),
            hil_enabled: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_HIL_ENABLED),
            manual_input_enabled: flag.intersects(
                mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_MANUAL_INPUT_ENABLED,
            ),
            safety_armed: flag
                .intersects(mavlink::ardupilotmega::MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED),
        }
    }

    pub fn decode_sensor_health(
        health: mavlink::ardupilotmega::MavSysStatusSensor,
    ) -> QuadSensorStatus {
        QuadSensorStatus {
            gyro: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO),
            accel: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL),
            mag: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG),
            abs_pressure: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE),
            diff_pressure: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE),
            gps: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_GPS),
            optical_flow: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW),
            vision_position: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_VISION_POSITION),
            laser_position: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_LASER_POSITION),
            external_ground_truth: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_EXTERNAL_GROUND_TRUTH),
            rate_control: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ANGULAR_RATE_CONTROL),
            attitude_stabilization: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ATTITUDE_STABILIZATION),
            yaw_position: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_YAW_POSITION),
            altitude_control: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_Z_ALTITUDE_CONTROL),
            xy_position_control: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_XY_POSITION_CONTROL),
            motor_control: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS),
            rc_receiver: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_RC_RECEIVER),
            gyro2: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO2),
            accel2: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL2),
            mag2: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG2),
            geofence: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_GEOFENCE),
            ahrs: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_AHRS),
            terrain: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_TERRAIN),
            reverse_motor: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_REVERSE_MOTOR),
            logging: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_LOGGING),
            battery: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_BATTERY),
            proximity: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_PROXIMITY),
            satcom: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_SATCOM),
            prearm_check: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_PREARM_CHECK),
            obstacle_avoidance: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_OBSTACLE_AVOIDANCE),
            propulsion: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_PROPULSION),
            extension: health.intersects(mavlink::ardupilotmega::MavSysStatusSensor::MAV_SYS_STATUS_EXTENSION_USED),
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

    pub fn decode_ekf_status(flags: mavlink::ardupilotmega::EkfStatusFlags) -> QuadEkfStatus {
        QuadEkfStatus {
            attitude: flags.intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_ATTITUDE),
            vel_horiz: flags.intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_VELOCITY_HORIZ),
            vel_vert: flags.intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_VELOCITY_VERT),
            pos_horiz_rel: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_POS_HORIZ_REL),
            pos_horiz_abs: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_POS_HORIZ_ABS),
            pos_vert_abs: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_POS_VERT_ABS),
            pos_vert_agl: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_POS_VERT_AGL),
            const_pos_mode: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_CONST_POS_MODE),
            pred_pos_horiz_rel: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_PRED_POS_HORIZ_REL),
            pred_pos_horiz_abs: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_PRED_POS_HORIZ_ABS),
            uninitialized: flags
                .intersects(mavlink::ardupilotmega::EkfStatusFlags::EKF_UNINITIALIZED),
        }
    }
}
