use std::collections::HashMap;

use egui::{ScrollArea, Ui};
use egui_plot::{Line, PlotPoints, Plot};
use log::info;

use super::DockableWidget;

use crate::viz::context::VizContext;

pub struct TelemetryGraphWidget {
    pub data: HashMap<String, Vec<f64>>,
    pub limit : u64,
}

impl TelemetryGraphWidget {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            limit: 1000,
        }
    }
}

impl DockableWidget for TelemetryGraphWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let sim_state = context.sim_state.clone();

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
                        let telem = state.uav_state.uav_state.telemtry;
                        // Limit 
                        ui.add(egui::Slider::new(&mut self.limit, 100..=5000).text("Limit"));

                        for (name, data) in telem {
                            match data.value {
                                crate::types::telemtry::TelemtryType::String(_) => todo!(),
                                crate::types::telemtry::TelemtryType::Float(v) => {
                                    // Draw a checkbox for each line
                                    let mut enabled = self.data.contains_key(name.as_str());
                                    ui.checkbox(&mut enabled, name.as_str());
                                    // Limit selector (default 1000)
                                 
                                    
                                    if enabled {
                                        // If the checkbox was checked, add the line to the plot
                                        let data = self.data.entry(name.clone()).or_default();
                                        data.push(v);
                                        // Limit to 10000 points
                                        if data.len() > self.limit as usize {
                                            data.remove(0);
                                        }
                                    } else {
                                        // If the checkbox was unchecked, remove the line from the plot
                                        self.data.remove(name.as_str());
                                    }

                                    ui.horizontal(|ui| {
                                         // Show current value
                                    ui.label(format!("{:.2}", v));

                                    // Clear button
                                    if ui.button("Clear").clicked() {
                                        self.data.remove(name.as_str());
                                    }
                                    });

                                    ui.separator();
                                }
                            }
                            
                        }
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
