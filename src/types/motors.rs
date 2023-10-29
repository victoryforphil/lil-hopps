use nalgebra::Vector3;

use crate::uav::config::UAVConfig;



pub struct Motor{
    pub motor_force_n: f32,
    pub motor_number: u8,
    pub motor_offset_b: Vector3<f32>,
    pub current_value: f32,
}
#[derive(Debug, PartialEq, Clone)]
pub struct MotorPhysics{
    pub force: Vector3<f32>,
    pub torque: Vector3<f32>,
    pub offset: Vector3<f32>,
}

impl std::fmt::Display for MotorPhysics{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "Force: {:?}, Torque: {:?}, Offset: {:?}", self.force, self.torque, self.offset)
    }
    
}

impl Motor{
    pub fn new(num:u8, config: UAVConfig) -> Motor{
        Motor{
            motor_force_n: config.motor_force_n,
            motor_number: num,
            motor_offset_b: Motor::calculate_offset(config.arm_length_m, num),
            current_value: 0.0,
        }
    }

    /// Set the input value of the motor (0.0 - 1.0)
    /// This is the value that is used to calculate the force vector
    /// 
    /// # Arguments
    /// 
    /// * `input` - The input value of the motor (0.0 - 1.0)
    pub fn set_input_scalar(&mut self, input: f32){
        self.current_value = input;
    }

    /// Calculate the force vector of the motor in the body frame 
    /// using the current value of the motor (0.0 - 1.0)
    /// (set using set_input_scalar() )
    /// 
    /// # Returns
    /// 
    pub fn get_physics(&self) -> MotorPhysics{
       let force =  self.current_value * self.motor_force_n;
       let force_vector = Vector3::new(0.0, 0.0, -force);
       let offset = self.motor_offset_b;
       let torque = Vector3::zeros(); //TODO: Motor Torque https://github.com/victoryforphil/lil-hopps/issues/7

         MotorPhysics{
              force: force_vector,
              torque: torque,
              offset: offset,
         }
    }

    ///
    /// Calculate the offset of the motor from the center of the UAV
    /// Use Following Motor Layer
    ///        
    ///        1     
    ///        |     
    ///   4 -- + -- 2         
    ///        |     
    ///       3     
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
        let config = UAVConfig { motor_force_n: 1000.0, arm_length_m: 125.0, weight_g: 500.0 };
        let motor = Motor::new(1, config);
        assert_eq!(motor.motor_force_n, 1000.0);
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

    #[test]
    fn test_set_input_scalar(){
        let config = UAVConfig::new_250mm();
        let mut motor = Motor::new(1, config);
        motor.set_input_scalar(0.5);
        assert_eq!(motor.current_value, 0.5);
    }

    #[test]
    fn test_get_force_vector(){
        let config = UAVConfig { motor_force_n: 1000.0, arm_length_m: 125.0, weight_g: 500.0 };
        let mut motor = Motor::new(1, config);
        motor.set_input_scalar(0.5);
        let force = motor.get_physics();
        assert_eq!(force.force, Vector3::new(0.0, 0.0, -500.0));
        //assert_eq!(force.torque, Vector3::new(0.0, 0.0, 62500.0)); TODO: Motor Torque https://github.com/victoryforphil/lil-hopps/issues/7
        assert_eq!(force.offset, Vector3::new(125.0, 0.0, 0.0));
    }
}