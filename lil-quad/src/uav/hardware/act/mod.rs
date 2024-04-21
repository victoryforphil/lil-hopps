use std::collections::BTreeMap;

use lil_broker::{DataPoint, Primatives, Timestamp};
#[derive(Debug, Clone)]
pub struct ActHardwareInputs{
    pub motor_messages: BTreeMap<String, Vec<DataPoint>>,
}
#[derive(Debug, Clone)]
pub struct ActHardwareOutput{
    pub debug_messages: BTreeMap<String, Vec<DataPoint>>,
}

pub trait ActHardware{
    fn act(&self, t:&Timestamp, inputs:ActHardwareInputs) -> Result<ActHardwareOutput, anyhow::Error>;
}

// --- Mock Section --- // 

/// Mock implementation of the ActHardware trait
/// Inputs:
/// - motor_messages: A map of motor names to a list of DataPoints
///     - `/motor/0`: [DataPoint::Number]
pub struct MockActHardware{
    
}

impl ActHardware for MockActHardware{
    fn act(&self, t:&Timestamp, inputs:ActHardwareInputs) -> Result<ActHardwareOutput, anyhow::Error>{
        let mut output = ActHardwareOutput{
            debug_messages: BTreeMap::new(),
        };

        let motor0 = inputs.motor_messages.get("/motor/0").unwrap();
        let motor0_data = motor0.last().unwrap().data.clone();
        let motor0_value = match motor0_data{
            Primatives::Number(n) => n,
            _ => return Err(anyhow::anyhow!("Expected Number, got {:?}", motor0_data)),
        };

        let debug_message = format!("motor_0={}", motor0_value);
        let debug_message_dp = DataPoint::new(t.clone(), Primatives::String(debug_message));
        output.debug_messages.insert("/debug/0".into(), vec![debug_message_dp]);
        Ok(output)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use pretty_assertions::{assert_eq};
    #[test]
    fn test_mock_act_hardware(){
        let mock = MockActHardware{};
        let t = Timestamp::new(0);
        let inputs = ActHardwareInputs{
            motor_messages: {
                let mut map = BTreeMap::new();
                map.insert("/motor/0".into(), vec![DataPoint::new(t, 5.0.into())]);
                map
            }
        };

        let output = mock.act(&t, inputs).unwrap();
        let debug_messages = output.debug_messages.get("/debug/0").unwrap();
        assert_eq!(debug_messages.len(), 1);
        let debug_message = &debug_messages[0];
        assert_eq!(debug_message.data, Primatives::String("motor_0=5".into()));
    }
}
