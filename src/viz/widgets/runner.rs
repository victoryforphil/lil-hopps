use std::fmt::format;

use egui::Ui;

use super::DockableWidget;

use crate::viz::context::VizContext;

pub struct RunnerWidget {}

impl RunnerWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl DockableWidget for RunnerWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        ui.label("Runner Widget");
        let mut runner = context.runner_handle.lock().unwrap();
        if ui.button("Start").clicked() {
            runner.start();
        }

        let runner_update = context.runner_update.clone();
        let sim_state = match runner_update {
            Some(update) => update.state_sample,
            None => None,
        };
       
        // UI for setting delta time
        ui.horizontal(|ui| {
            ui.label("dt: ");
            let _dt = ui 
                .add(egui::DragValue::new(&mut runner.options.dt).speed(0.001));
            
            
                
        });

        // ui for setting max t
        ui.horizontal(|ui| {
            ui.label("max_t: ");
            let _max_t = ui
                .add(egui::DragValue::new(&mut runner.options.max_t).speed(0.1));
            
        });
        if let Some(state) = sim_state {

            let running = state.running;

            // Print Green lable if running else red
            if running {
                ui.colored_label(egui::Color32::GREEN, "Running");
            } else {
                ui.colored_label(egui::Color32::RED, "Stopped");
            }

            ui.label(state.time.to_string());
            ui.label(format!("Max T: {}", runner.options.max_t));
            
        }
    }
}
