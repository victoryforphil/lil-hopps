use nalgebra::Vector3;

use crate::uav::config::UAVConfig;



pub struct Motor{
    pub motor_force_n: f32,
    pub motor_number: u8,
    pub motor_offset_b: Vector3<f32>

}

impl Motor{
    pub fn new(num:u8, config: UAVConfig) -> Motor{
        Motor{
            motor_force_n: config.motor_force_n,
            motor_number: num,
            motor_offset_b: Motor::calculate_offset(config.arm_length_m, num),
        }
    }
    ///
    /// Calculate the offset of the motor from the center of the UAV
    /// Use Following Motor Layer
    ///        
    ///        1     
    ///        |     
    ///   4 -- + -- 2     
    ///       |     
    ///      3     
    /// 
    /// 1 - Front 
    /// 2 - Right
    /// 3 - Back
    /// 4 - Left
    pub fn calculate_offset(arm_size: f32, motor_num:u8) -> Vector3<f32>{
        match motor_num{
            1 => Vector3::new(arm_size, 0.0, 0.0),
            2 => Vector3::new(0.0, -arm_size, 0.0),
            3 => Vector3::new(-arm_size, 0.0, 0.0),
            4 => Vector3::new(0.0, -arm_size, 0.0),
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let config = UAVConfig::new_250mm();
        let motor = Motor::new(1, config);
        assert_eq!(motor.motor_force_n, 2000.0);
        assert_eq!(motor.motor_number, 1);
        assert_eq!(motor.motor_offset_b, Vector3::new(125.0, 0.0, 0.0));
    }

    #[test]
    fn test_calculate_offset(){
        let offset = Motor::calculate_offset(125.0, 1);
        assert_eq!(offset, Vector3::new(125.0, 0.0, 0.0));

        let offset = Motor::calculate_offset(125.0, 2);
        assert_eq!(offset, Vector3::new(0.0, -125.0, 0.0));

        let offset = Motor::calculate_offset(125.0, 3);
        assert_eq!(offset, Vector3::new(-125.0, 0.0, 0.0));

        let offset = Motor::calculate_offset(125.0, 4);
        assert_eq!(offset, Vector3::new(0.0, -125.0, 0.0));
    }
}