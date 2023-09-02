pub trait TelemtryHardware{
    fn send_message(&mut self, message: String) -> Result<(), String>;
    fn get_messages(&self) -> Result<Vec<String>, String>;
}

#[cfg(test)]
mod tests{
    use super::*;
    struct MockTelemetryHardware{
        messages: Vec<String>
    }

    impl MockTelemetryHardware{
        fn new(messages: Vec<String>) -> MockTelemetryHardware{
            MockTelemetryHardware{
                messages
            }
        }
    }

    impl TelemtryHardware for MockTelemetryHardware{
        fn send_message(&mut self, message: String) -> Result<(), String>{
            self.messages.push(message);
            Ok(())
        }

        fn get_messages(&self) -> Result<Vec<String>, String>{
            Ok(self.messages.clone())
        }
    }

    #[test]
    fn test_mock_telemetry_hardware(){
        let messages = vec![String::from("Hello World")];
        let mut mock_telemetry_hardware = MockTelemetryHardware::new(messages.clone());
        let result = mock_telemetry_hardware.send_message(String::from("Hello World"));
        assert!(result.is_ok());

        let result = mock_telemetry_hardware.get_messages();
        assert!(result.is_ok());
        let result_messages = result.unwrap();
        assert_eq!(result_messages, [String::from("Hello World"), String::from("Hello World")]);
    }
}