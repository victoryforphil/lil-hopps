use log::info;

use crate::{types::telemtry::{Telemtry, TelemtryType}, control::UAVController};

use super::state::UAVState;

pub struct UAVSoftware {
    control: UAVController
}

impl UAVSoftware {
    pub fn new() -> Self {
        UAVSoftware {
            control: UAVController::new()
        }
    }

    pub fn process(&mut self, _t: f64, dt: f32, in_state: &UAVState) -> Result<UAVState, String> {
        let mut state = in_state.clone();

        let motors = self.control.update(&state, dt as f64);
        state.safe_set_motors([motors[0] as f32, motors[1] as f32, motors[2] as f32, motors[3] as f32]);


        self.update_telemetry(&mut state);
        Ok(state)
    }

    fn update_telemetry(&self, state: &mut UAVState) {
        self.add_telemetry(state, "pose_pos_x", TelemtryType::Float(state.pose.position.x.into()));
        self.add_telemetry(state, "pose_pos_y", TelemtryType::Float(state.pose.position.y.into()));
        self.add_telemetry(state, "pose_pos_z", TelemtryType::Float(state.pose.position.z.into()));
        self.add_telemetry(state, "pose_ori_x_rad", TelemtryType::Float(state.pose.orientation.euler_angles().0.into()));
        self.add_telemetry(state, "pose_ori_y_rad", TelemtryType::Float(state.pose.orientation.euler_angles().1.into()));
        self.add_telemetry(state, "pose_ori_z_rad", TelemtryType::Float(state.pose.orientation.euler_angles().2.into()));
        self.add_telemetry(state, "motor_1", TelemtryType::Float(state.motors[0].current_value.into()));
        self.add_telemetry(state, "motor_2", TelemtryType::Float(state.motors[1].current_value.into()));
        self.add_telemetry(state, "motor_3", TelemtryType::Float(state.motors[2].current_value.into()));
        self.add_telemetry(state, "motor_4", TelemtryType::Float(state.motors[3].current_value.into()));
    }

    fn add_telemetry(&self, state: &mut UAVState, name: &str, value: TelemtryType) {
        let telemetry = Telemtry {
            name: name.to_string(),
            value,
        };
        state.telemtry.insert(telemetry.name.clone(), telemetry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{uav::{state::UAVState, config}, types::motors::Motor};

    #[test]
    fn test_uav_software() {
        let mut software = UAVSoftware::new();
        let config = config::UAVConfig::new_250mm();
        let state = UAVState::new(Motor::generate_motors(&config));
        let result = software.process(0.0, 0.0, &state);
        assert!(result.is_ok());
    }
}
