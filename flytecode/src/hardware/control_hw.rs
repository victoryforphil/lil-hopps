pub trait ControlHardware{
    fn set_motor_values(&mut self, motor_values: [f32; 4]) -> Result<(), String>;
    fn get_motor_values(&self) -> Result<[f32; 4], String>;
}

#[cfg(test)]
mod tests{
    use super::*;
    struct MockControlHardware{
        motor_values: [f32; 4]
    }

    impl MockControlHardware{
        fn new(motor_values :[f32;4]) -> MockControlHardware{
            MockControlHardware{
                motor_values
            }
        }
    }

    impl ControlHardware for MockControlHardware{
        fn set_motor_values(&mut self, motor_values: [f32; 4]) -> Result<(), String>{
            self.motor_values = motor_values;
            Ok(())
        }

        fn get_motor_values(&self) -> Result<[f32; 4], String>{
    
            Ok(self.motor_values.clone())
        }
    }

    #[test]
    fn test_mock_control_hardware(){
        let motor_values = [0.0;4];
        let mut mock_control_hardware = MockControlHardware::new(motor_values.clone());
        let result = mock_control_hardware.set_motor_values([1.0; 4]);
        assert!(result.is_ok());

        let result = mock_control_hardware.get_motor_values();
        assert!(result.is_ok());
        let result_motor_values = result.unwrap();
        assert_eq!(result_motor_values, [1.0; 4]);
    }
}