use std::collections::HashMap;

use crate::{
    logging::{LogData, LogEntry, Loggable},
    types::{motors::Motor, movement::Movement, pose::Pose, telemtry::Telemtry},
};
use polars::prelude::*;
use polars::functions::*;

use crate::types::{movement::Movement, pose::Pose, telemtry::Telemtry, motors::Motor};

#[derive(Debug, PartialEq, Clone)]
pub struct UAVState {
    pub pose: Pose,
    pub movenment: Movement,
    pub motors: [Motor; 4],
    pub telemtry: HashMap<String, Telemtry>,
}

impl Loggable for UAVState {
    fn log(&self, t: f64) -> Vec<crate::logging::LogEntry> {
        let mut entries = vec![];

        for motor in &self.motors {
            entries.push(LogEntry::new(
                format!("motor_{}", motor.motor_number),
                t,
                LogData::Motor(motor.clone()),
            ));
        }

        entries.push(LogEntry::new(
            "pose".to_string(),
            t,
            LogData::Pose(self.pose.clone()),
        ));

        entries.push(LogEntry::new(
            "movement".to_string(),
            t,
            LogData::Movement(self.movenment.clone()),
        ));

        return entries;
    }
}

impl UAVState {
    ///
    /// Creates a new UAVState with the given pose and zero movement and motors.
    ///
    pub fn new(motors: Vec<Motor>) -> Self {
        UAVState {
            pose: Pose::zero(),
            movenment: Movement::zero(),
            motors: [
                motors[0].clone(),
                motors[1].clone(),
                motors[2].clone(),
                motors[3].clone(),
            ],
            telemtry: HashMap::new(),
        }
    }

    ///
    /// Creates a new UAVState with the given pose and zero movement and motors.
    ///
    pub fn new_with_pose(init_pose: Pose, motors: Vec<Motor>) -> Self {
        UAVState {
            pose: init_pose,
            movenment: Movement::zero(),
            motors: [
                motors[0].clone(),
                motors[1].clone(),
                motors[2].clone(),
                motors[3].clone(),
            ],
            telemtry: HashMap::new(),
        }
    }

    // This function is used to set the motors of the UAV.
    // The motors are clamped to the range [0, 1].
    pub fn safe_set_motors(&mut self, motors: [f32; 4]) {
        for i in 0..4 {
            self.motors[i].set_input_scalar(motors[i].max(0.0).min(1.0));
        }
    }

    pub fn get_df(&self, lable:String) -> DataFrame{
        // Get dataframes from pose motors and movement and merge them diagonal into one flat dataframe
        let mut dfs = vec![];

        dfs.push(self.pose.get_df(format!("{}.pose", lable)));
        dfs.push(self.movenment.get_df(format!("{}.movement", lable)));
     
        for i in 0..4 {
           dfs.push(self.motors[i].get_df(format!("{}.motor_{}", lable, i)));
        }
        concat_df_horizontal(dfs.as_slice()).unwrap()

        
    }
}

#[cfg(test)]
mod tests {
    use crate::uav::config::{self, UAVConfig};

    use super::*;

    #[test]
    fn test_new() {
        let config = config::UAVConfig::new_250mm();
        let state = UAVState::new(Motor::generate_motors(&config));
        assert_eq!(state.pose, Pose::zero());
        assert_eq!(state.movenment, Movement::zero());
        assert_eq!(state.motors.len(), 4);
    }

    #[test]
    fn test_new_pose() {
        let pose = Pose::zero();
        let config = config::UAVConfig::new_250mm();

        let state = UAVState::new_with_pose(pose, Motor::generate_motors(&config));
        assert_eq!(state.pose, pose);
        assert_eq!(state.movenment, Movement::zero());
        assert_eq!(state.motors.len(), 4);
    }

    #[test]
    fn test_safe_set_motors() {
        let config = UAVConfig::new_250mm();
        let mut state = UAVState::new(Motor::generate_motors(&config));
        state.safe_set_motors([0.0, 0.5, 1.0, 1.5]);
        assert_eq!(state.motors.len(), 4);
    }
}
