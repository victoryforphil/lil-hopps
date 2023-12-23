use std::collections::HashMap;

use egui::{ScrollArea, Ui};
use egui_plot::{Line, PlotPoints, Plot};
use log::info;

use super::DockableWidget;

use crate::viz::context::VizContext;

pub struct DataframeGraphWidget {
    pub data: HashMap<String, Vec<f64>>,
    pub limit : u64,
}

impl DataframeGraphWidget {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            limit: 1000,
        }
    }
}

impl DockableWidget for DataframeGraphWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let runner_update = context.runner_update.clone();
        let (sim_state, dataframe) = match runner_update {
            Some(update) => (update.state_sample, update.df),
            None => (None, None),
        };

        if let Some(state) = sim_state {
            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(75.0)
                .width_range(20.0..=800.0)
                .show_inside(ui, |ui| {
                    let scroll_area = ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .stick_to_bottom(true);
                    scroll_area.show(ui, |ui| {

                        
                    });
                });
                let mut lines = vec![];
                for (name, data) in &self.data {
                    let mut t = 0;
                    let sin: PlotPoints = data.into_iter().map(|v| {
                        t += 1;
                        [t as f64, *v]
                    }).collect();
                    let line = Line::new(sin);
                    lines.push(line);
                }
                Plot::new("my_plot").show(ui, |plot_ui| {
                    for line in lines {
                        
                        plot_ui.line(line);
                    }
                });
        }
    }
}
