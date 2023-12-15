use egui::Ui;

use super::DockableWidget;

use crate::{viz::context::VizContext, uav};

pub struct UAVStateWidget {}

impl UAVStateWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl DockableWidget for UAVStateWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let sim_state = context.sim_state.clone();

        if let Some(state) = sim_state {
            let uav_pose = state.uav_state.uav_state.pose;
            let uav_motors = state.uav_state.uav_state.motors;
            let uav_movement = state.uav_state.uav_state.movenment;

            ui.label("UAV Pose");
            ui.label("X: ".to_string() + &uav_pose.position.x.to_string());
            ui.label("Y: ".to_string() + &uav_pose.position.y.to_string());
            ui.label("Z: ".to_string() + &uav_pose.position.z.to_string());
            ui.separator();

            ui.label("UAV Orientation");
            ui.label("X: ".to_string() + &uav_pose.orientation.euler_angles().0.to_string());
            ui.label("Y: ".to_string() + &uav_pose.orientation.euler_angles().1.to_string());
            ui.label("Z: ".to_string() + &uav_pose.orientation.euler_angles().2.to_string());
            ui.separator();

            ui.label("UAV Ang Acc");
            ui.label("X / Roll: ".to_string() + &uav_movement.ang_accel[0].to_string());
            ui.label("Y / Pitch: ".to_string() + &uav_movement.ang_accel[1].to_string());
            ui.label("Z / Yaw: ".to_string() + &uav_movement.ang_accel[2].to_string());
            ui.separator();

            ui.label("UAV Ang Vel");
            ui.label("X / Roll: ".to_string() + &uav_movement.ang_vel[0].to_string());
            ui.label("Y / Pitch: ".to_string() + &uav_movement.ang_vel[1].to_string());
            ui.label("Z / Yaw: ".to_string() + &uav_movement.ang_vel[2].to_string());

            ui.label("UAV Motors");
            ui.label("M1: ".to_string() + &uav_motors[0].to_string());
            ui.label("M2: ".to_string() + &uav_motors[1].to_string());
            ui.label("M3: ".to_string() + &uav_motors[2].to_string());
            ui.label("M4: ".to_string() + &uav_motors[3].to_string());
        }
    }
}
