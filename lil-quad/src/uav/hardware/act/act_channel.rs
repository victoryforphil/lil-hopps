use crossbeam_channel::{Receiver, Sender};

use super::{ActHardware, ActHardwareInputs, ActHardwareOutput};

pub struct ChannelAct {
    pub rx: Receiver<ActHardwareInputs>,
    pub tx: Sender<ActHardwareInputs>,
}

impl ChannelAct {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self { rx, tx }
    }
}

impl ActHardware for ChannelAct {
    fn act(
        &self,
        _t: &lil_broker::Timestamp,
        inputs: super::ActHardwareInputs,
    ) -> Result<super::ActHardwareOutput, anyhow::Error> {
        self.tx.send(inputs).unwrap();
        Ok(ActHardwareOutput {
            debug_messages: std::collections::BTreeMap::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lil_broker::DataPoint;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn test_channel_act() {
        let channel_act = ChannelAct::new();
        let t = lil_broker::Timestamp::new(0);
        let inputs = ActHardwareInputs {
            motor_messages: {
                let mut map = BTreeMap::new();
                map.insert(
                    "/motor/0".into(),
                    vec![DataPoint::new(t.clone(), 5.0.into())],
                );
                map
            },
        };
        let output = channel_act.act(&t, inputs).unwrap();
        assert_eq!(output.debug_messages.len(), 0);
        let received_inputs = channel_act.rx.recv().unwrap();
        assert_eq!(received_inputs.motor_messages.len(), 1);
        assert_eq!(
            received_inputs
                .motor_messages
                .get("/motor/0")
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            received_inputs.motor_messages.get("/motor/0").unwrap()[0].data,
            lil_broker::Primatives::Number(5.0)
        );
    }
}
