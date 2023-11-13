use std::time::Duration;

use super::{context::VizContext, widgets::WidgetUI};

pub struct VisualizationApp {
    pub widget_ui: WidgetUI,
    pub context: VizContext,
}

impl VisualizationApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, viz_context: VizContext) -> Self {
        Self {
            widget_ui: WidgetUI::new(viz_context.clone()),
            context: viz_context,
        }
    }

    pub fn init(mut self) -> VisualizationApp {
        self
    }
}

impl eframe::App for VisualizationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.context.update_sim_state();
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Start").clicked() {
                self.context.runner_handle.lock().unwrap().start();
            }

            let sim_state = self.context.sim_state.clone();

            if let Some(state) = sim_state {
                let running = state.running;

                // Print Green lable if running else red
                if running {
                    ui.colored_label(egui::Color32::GREEN, "Running");
                } else {
                    ui.colored_label(egui::Color32::RED, "Stopped");
                }

                ui.label(state.uav_state.uav_state.pose.to_string());
            }
        });

        ctx.request_repaint();
    }
}
