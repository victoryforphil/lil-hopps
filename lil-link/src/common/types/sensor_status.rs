use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuadSensorStatus {
    pub gyro: bool,                   // MAV_SYS_STATUS_SENSOR_3D_GYRO (0x01)
    pub accel: bool,                  // MAV_SYS_STATUS_SENSOR_3D_ACCEL (0x02)
    pub mag: bool,                    // MAV_SYS_STATUS_SENSOR_3D_MAG (0x04)
    pub abs_pressure: bool,           // MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE (0x08)
    pub diff_pressure: bool,          // MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE (0x10)
    pub gps: bool,                    // MAV_SYS_STATUS_SENSOR_GPS (0x20)
    pub optical_flow: bool,           // MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW (0x40)
    pub vision_position: bool,        // MAV_SYS_STATUS_SENSOR_VISION_POSITION (0x80)
    pub laser_position: bool,         // MAV_SYS_STATUS_SENSOR_LASER_POSITION (0x100)
    pub external_ground_truth: bool,  // MAV_SYS_STATUS_SENSOR_EXTERNAL_GROUND_TRUTH (0x200)
    pub rate_control: bool,           // MAV_SYS_STATUS_SENSOR_ANGULAR_RATE_CONTROL (0x400)
    pub attitude_stabilization: bool, // MAV_SYS_STATUS_SENSOR_ATTITUDE_STABILIZATION (0x800)
    pub yaw_position: bool,           // MAV_SYS_STATUS_SENSOR_YAW_POSITION (0x1000)
    pub altitude_control: bool,       // MAV_SYS_STATUS_SENSOR_Z_ALTITUDE_CONTROL (0x2000)
    pub xy_position_control: bool,    // MAV_SYS_STATUS_SENSOR_XY_POSITION_CONTROL (0x4000)
    pub motor_control: bool,          // MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS (0x8000)
    pub rc_receiver: bool,            // MAV_SYS_STATUS_SENSOR_RC_RECEIVER (0x10000)
    pub gyro2: bool,                  // MAV_SYS_STATUS_SENSOR_3D_GYRO2 (0x20000)
    pub accel2: bool,                 // MAV_SYS_STATUS_SENSOR_3D_ACCEL2 (0x40000)
    pub mag2: bool,                   // MAV_SYS_STATUS_SENSOR_3D_MAG2 (0x80000)
    pub geofence: bool,               // MAV_SYS_STATUS_GEOFENCE (0x100000)
    pub ahrs: bool,                   // MAV_SYS_STATUS_AHRS (0x200000)
    pub terrain: bool,                // MAV_SYS_STATUS_TERRAIN (0x400000)
    pub reverse_motor: bool,          // MAV_SYS_STATUS_REVERSE_MOTOR (0x800000)
    pub logging: bool,                // MAV_SYS_STATUS_LOGGING (0x1000000)
    pub battery: bool,                // MAV_SYS_STATUS_SENSOR_BATTERY (0x2000000)
    pub proximity: bool,              // MAV_SYS_STATUS_SENSOR_PROXIMITY (0x4000000)
    pub satcom: bool,                 // MAV_SYS_STATUS_SENSOR_SATCOM (0x8000000)
    pub prearm_check: bool,           // MAV_SYS_STATUS_PREARM_CHECK (0x10000000)
    pub obstacle_avoidance: bool,     // MAV_SYS_STATUS_OBSTACLE_AVOIDANCE (0x20000000)
    pub propulsion: bool,             // MAV_SYS_STATUS_SENSOR_PROPULSION (0x40000000)
    pub extension: bool,              // MAV_SYS_STATUS_EXTENSION_USED (0x80000000)
}

impl QuadSensorStatus {
    pub fn get_mavlink_map(&self) -> HashMap<String, bool> {
        let mut map = HashMap::new();
        map.insert("MAV_SYS_STATUS_SENSOR_3D_GYRO".to_string(), self.gyro);
        map.insert("MAV_SYS_STATUS_SENSOR_3D_ACCEL".to_string(), self.accel);
        map.insert("MAV_SYS_STATUS_SENSOR_3D_MAG".to_string(), self.mag);
        map.insert(
            "MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE".to_string(),
            self.abs_pressure,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE".to_string(),
            self.diff_pressure,
        );
        map.insert("MAV_SYS_STATUS_SENSOR_GPS".to_string(), self.gps);
        map.insert(
            "MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW".to_string(),
            self.optical_flow,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_VISION_POSITION".to_string(),
            self.vision_position,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_LASER_POSITION".to_string(),
            self.laser_position,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_EXTERNAL_GROUND_TRUTH".to_string(),
            self.external_ground_truth,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_ANGULAR_RATE_CONTROL".to_string(),
            self.rate_control,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_ATTITUDE_STABILIZATION".to_string(),
            self.attitude_stabilization,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_YAW_POSITION".to_string(),
            self.yaw_position,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_Z_ALTITUDE_CONTROL".to_string(),
            self.altitude_control,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_XY_POSITION_CONTROL".to_string(),
            self.xy_position_control,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS".to_string(),
            self.motor_control,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_RC_RECEIVER".to_string(),
            self.rc_receiver,
        );
        map.insert("MAV_SYS_STATUS_SENSOR_3D_GYRO2".to_string(), self.gyro2);
        map.insert("MAV_SYS_STATUS_SENSOR_3D_ACCEL2".to_string(), self.accel2);
        map.insert("MAV_SYS_STATUS_SENSOR_3D_MAG2".to_string(), self.mag2);
        map.insert("MAV_SYS_STATUS_GEOFENCE".to_string(), self.geofence);
        map.insert("MAV_SYS_STATUS_AHRS".to_string(), self.ahrs);
        map.insert("MAV_SYS_STATUS_TERRAIN".to_string(), self.terrain);
        map.insert(
            "MAV_SYS_STATUS_REVERSE_MOTOR".to_string(),
            self.reverse_motor,
        );
        map.insert("MAV_SYS_STATUS_LOGGING".to_string(), self.logging);
        map.insert("MAV_SYS_STATUS_SENSOR_BATTERY".to_string(), self.battery);
        map.insert(
            "MAV_SYS_STATUS_SENSOR_PROXIMITY".to_string(),
            self.proximity,
        );
        map.insert("MAV_SYS_STATUS_SENSOR_SATCOM".to_string(), self.satcom);
        map.insert("MAV_SYS_STATUS_PREARM_CHECK".to_string(), self.prearm_check);
        map.insert(
            "MAV_SYS_STATUS_OBSTACLE_AVOIDANCE".to_string(),
            self.obstacle_avoidance,
        );
        map.insert(
            "MAV_SYS_STATUS_SENSOR_PROPULSION".to_string(),
            self.propulsion,
        );
        map.insert("MAV_SYS_STATUS_EXTENSION_USED".to_string(), self.extension);
        map
    }
}
