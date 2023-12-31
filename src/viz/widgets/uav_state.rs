use egui::Ui;

use super::DockableWidget;

use crate::viz::context::VizContext;

pub struct UAVStateWidget {}

impl UAVStateWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl DockableWidget for UAVStateWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let runner_update = context.runner_update.clone();
        let (sim_state,df) = match runner_update {
            Some(update) => (update.state_sample, update.df),
            None => return,
        };
       

        if let Some(state) = sim_state {
            
            let uav_pose = state.uav_state.uav_state.pose;
            let uav_motors = state.uav_state.uav_state.motors;

            ui.label("UAV Pose");
            ui.label(&format!("X: {}", uav_pose.position.x));
            ui.label(&format!("Y: {}", uav_pose.position.y));
            ui.label(&format!("Z: {}", uav_pose.position.z));
            ui.separator();

            ui.label("UAV Orientation");
            ui.label(&format!("X: {}", uav_pose.orientation.euler_angles().0));
            ui.label(&format!("Y: {}", uav_pose.orientation.euler_angles().1));
            ui.label(&format!("Z: {}", uav_pose.orientation.euler_angles().2));
            ui.separator();
            ui.label("UAV Motors");
            ui.label(&format!("M1: {:?}", uav_motors[0]));
            ui.label(&format!("M2: {:?}", uav_motors[1]));
            ui.label(&format!("M3: {:?}", uav_motors[2]));
            ui.label(&format!("M4: {:?}", uav_motors[3]));
        }

        if let Some(df) = df {
           // Print all latest dataframes
              for col in df.get_columns() {
                ui.label(&format!("{}: {}",col.name(), col.f32().unwrap().last().unwrap()));
              }
            
           
        }
    }
}
