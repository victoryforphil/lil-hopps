use log::info;

use crate::types::telemtry::{Telemtry, TelemtryType};

use super::state::UAVState;

pub struct UAVSoftware {}

impl UAVSoftware {
    pub fn new() -> Self {
        UAVSoftware {}
    }

    pub fn process(&mut self, _t: f64, _dt: f32, in_state: &UAVState) -> Result<UAVState, String> {
      
        let pose_pos_x_tel = Telemtry{
            name: "pose_pos_x".to_string(),
            value: TelemtryType::Float(in_state.pose.position.x.into())
        };
        let pose_pos_y_tel = Telemtry{
            name: "pose_pos_y".to_string(),
            value: TelemtryType::Float(in_state.pose.position.y.into())
        };
        let pose_pos_z_tel = Telemtry{
            name: "pose_pos_z".to_string(),
            value: TelemtryType::Float(in_state.pose.position.z.into())
        };

        let pose_ori_x_tel = Telemtry{
            name: "pose_ori_x_rad".to_string(),
            value: TelemtryType::Float(in_state.pose.orientation.euler_angles().0.into())
        };
        let pose_ori_y_tel = Telemtry{
            name: "pose_ori_y_rad".to_string(),
            value: TelemtryType::Float(in_state.pose.orientation.euler_angles().1.into())
        };
        let pose_ori_z_tel = Telemtry{
            name: "pose_ori_z_rad".to_string(),
            value: TelemtryType::Float(in_state.pose.orientation.euler_angles().2.into())
        };

        let motor_1_tel = Telemtry{
            name: "motor_1".to_string(),
            value: TelemtryType::Float(in_state.motors[0].current_value.into())
        };
        let motor_2_tel = Telemtry{
            name: "motor_2".to_string(),
            value: TelemtryType::Float(in_state.motors[1].current_value.into())
        };
        let motor_3_tel = Telemtry{
            name: "motor_3".to_string(),
            value: TelemtryType::Float(in_state.motors[2].current_value.into())
        };
        let motor_4_tel = Telemtry{
            name: "motor_4".to_string(),
            value: TelemtryType::Float(in_state.motors[3].current_value.into())
        };

        let mut state = in_state.clone();
        state.telemtry.insert(pose_pos_x_tel.name.clone(), pose_pos_x_tel);
        state.telemtry.insert(pose_pos_y_tel.name.clone(), pose_pos_y_tel);
        state.telemtry.insert(pose_pos_z_tel.name.clone(), pose_pos_z_tel);
        state.telemtry.insert(pose_ori_x_tel.name.clone(), pose_ori_x_tel);
        state.telemtry.insert(pose_ori_y_tel.name.clone(), pose_ori_y_tel);
        state.telemtry.insert(pose_ori_z_tel.name.clone(), pose_ori_z_tel);
        state.telemtry.insert(motor_1_tel.name.clone(), motor_1_tel);
        state.telemtry.insert(motor_2_tel.name.clone(), motor_2_tel);
        state.telemtry.insert(motor_3_tel.name.clone(), motor_3_tel);
        state.telemtry.insert(motor_4_tel.name.clone(), motor_4_tel);
       
        Ok(state)
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
