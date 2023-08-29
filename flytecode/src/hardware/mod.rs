pub mod control_hw;
pub mod navigation_hw;
pub mod telemetry_hw;

// Base hardware trait

pub trait Hardware{
    fn init(&mut self) -> Result<(), String>;
    fn update(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn get_status(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::sync::{Arc, Mutex};
    struct MockHardware{
        status: Arc<Mutex<String>>
    }

    impl MockHardware{
        fn new(status: Arc<Mutex<String>>) -> MockHardware{
            MockHardware{
                status
            }
        }
    }

    impl Hardware for MockHardware{
        fn init(&mut self) -> Result<(), String>{
            let mut status = self.status.lock().unwrap();
            *status = String::from("Initialized");
            Ok(())
        }

        fn update(&mut self) -> Result<(), String>{
            let mut status = self.status.lock().unwrap();
            *status = String::from("Updated");
            Ok(())
        }

        fn shutdown(&mut self) -> Result<(), String>{
            let mut status = self.status.lock().unwrap();
            *status = String::from("Shutdown");
            Ok(())
        }

        fn get_status(&self) -> Result<(), String>{
            let status = self.status.lock().unwrap();
            println!("{}", status);
            Ok(())
        }
    }

    #[test]
    fn test_mock_hardware(){
        let status = Arc::new(Mutex::new(String::from("")));
        let mut mock_hardware = MockHardware::new(status.clone());
        let result = mock_hardware.init();
        assert!(result.is_ok());
        let result = mock_hardware.get_status();
        assert!(result.is_ok());
        let result = mock_hardware.update();
        assert!(result.is_ok());
        let result = mock_hardware.get_status();
        assert!(result.is_ok());
        let result = mock_hardware.shutdown();
        assert!(result.is_ok());
        let result = mock_hardware.get_status();
        assert!(result.is_ok());
    }

}