use std::collections::BTreeMap;

use lil_broker::{DataPoint, Timestamp};


#[derive(Debug,Clone)]
pub struct SenseHardwareOutputs{
    pub nav_messages: BTreeMap<String, Vec<DataPoint>>,
    pub telemetry_messages: BTreeMap<String, Vec<DataPoint>>,
    pub debug_messages: BTreeMap<String, Vec<DataPoint>>,
}

pub trait SenseHardware{
    fn sense(&self) -> Result<SenseHardwareOutputs, anyhow::Error>;
}

// --- Mock Section --- //
/// Mock implementation of the SenseHardware trait
/// Outputs:
/// - nav_messages: A map of nav messages to a list of DataPoints
///    - `/nav/0`: [DataPoint::Number]
/// - telemetry_messages: A map of telemetry messages to a list of DataPoints
///   - `/telemetry/0`: [DataPoint::Number]
/// - debug_messages: A map of debug messages to a list of DataPoints
///  - `/debug/0`: [DataPoint::String]
/// 
pub struct MockSenseHardware{
    
}

impl SenseHardware for MockSenseHardware{
    fn sense(&self) -> Result<SenseHardwareOutputs, anyhow::Error>{
        let mut outputs = SenseHardwareOutputs{
            nav_messages: BTreeMap::new(),
            telemetry_messages: BTreeMap::new(),
            debug_messages: BTreeMap::new(),
        };

        let nav_message = DataPoint::new(Timestamp::zero(), 1.0.into());
        outputs.nav_messages.insert("/nav/0".into(), vec![nav_message]);

        let telemetry_message = DataPoint::new(Timestamp::zero(), 20.0.into());
        outputs.telemetry_messages.insert("/telemetry/0".into(), vec![telemetry_message]);

        let debug_message = DataPoint::new(Timestamp::zero(), "lil-hopps".to_string().into());
        outputs.debug_messages.insert("/debug/0".into(), vec![debug_message]);

        Ok(outputs)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use lil_broker::Primatives;
    use pretty_assertions::{assert_eq};
    #[test]
    fn test_mock_sense_hardware(){
        let mock = MockSenseHardware{};
        let outputs = mock.sense().unwrap();

        assert_eq!(outputs.nav_messages.len(), 1);
        assert_eq!(outputs.telemetry_messages.len(), 1);
        assert_eq!(outputs.debug_messages.len(), 1);

        let nav_messages = outputs.nav_messages.get("/nav/0").unwrap();
        assert_eq!(nav_messages.len(), 1);
        let nav_message = &nav_messages[0];
        assert_eq!(nav_message.data, Primatives::Number(1.0));
        
        let telemetry_messages = outputs.telemetry_messages.get("/telemetry/0").unwrap();
        assert_eq!(telemetry_messages.len(), 1);
        let telemetry_message = &telemetry_messages[0];
        assert_eq!(telemetry_message.data, Primatives::Number(20.0));

        let debug_messages = outputs.debug_messages.get("/debug/0").unwrap();
        assert_eq!(debug_messages.len(), 1);
        let debug_message = &debug_messages[0];
        assert_eq!(debug_message.data, Primatives::String("lil-hopps".into()));
    }
}
