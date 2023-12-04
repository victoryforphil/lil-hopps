

use log::debug;

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
        debug!("[VisualizationApp] init");
        self.widget_ui.create();
        self
    }
}

impl eframe::App for VisualizationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.context.update_sim_state();
        egui::CentralPanel::default().show(ctx, |ui| {
            /*

            */

            self.widget_ui.draw_inside(ctx, ui, &self.context);
        });

        ctx.request_repaint();
    }
}
