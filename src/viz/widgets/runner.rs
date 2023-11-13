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

        let sim_state = context.sim_state.clone();

        if let Some(state) = sim_state {
            let running = state.running;

            // Print Green lable if running else red
            if running {
                ui.colored_label(egui::Color32::GREEN, "Running");
            } else {
                ui.colored_label(egui::Color32::RED, "Stopped");
            }

            ui.label(state.time.to_string());
            ui.label("Max time: ".to_string() + &runner.options.max_t.to_string());
        }
    }
}
