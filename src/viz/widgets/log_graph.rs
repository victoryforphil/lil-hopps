use std::collections::HashMap;

use egui::{ScrollArea, Ui};
use egui_plot::{Line, Plot, PlotPoints};


use super::DockableWidget;

use crate::viz::context::VizContext;

pub struct LogGraphWidget {
    pub data: HashMap<String, Vec<f32>>,
    pub limit: u64,
}

impl LogGraphWidget {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            limit: 1000,
        }
    }

    //batch version
    fn get_floats_for_entries(
        &self,
        entries: Vec<crate::logging::LogEntry>,
    ) -> HashMap<String, Vec<f32>> {
        let mut floats: HashMap<String, Vec<f32>> = HashMap::new();
        for entry in entries {
            let key = entry.key.clone();
            match entry.data {
                crate::logging::LogData::Float(v) => {
                    if let Some(existing_floats) = floats.get_mut(&key) {
                        existing_floats.push(v);
                    } else {
                        floats.insert(key, vec![v]);
                    }
                }
                crate::logging::LogData::Movement(_m) => {}
                crate::logging::LogData::Pose(p) => {
                    let pos_x_key = format!("{}/{}/{}", key, "pos", "x").to_string();
                    let pos_y_key = format!("{}/{}/{}", key, "pos", "y").to_string();
                    let pos_z_key = format!("{}/{}/{}", key, "pos", "z").to_string();
                    floats
                        .entry(pos_x_key)
                        .or_insert_with(Vec::new)
                        .push(p.position.x);
                    floats
                        .entry(pos_y_key)
                        .or_insert_with(Vec::new)
                        .push(p.position.y);
                    floats
                        .entry(pos_z_key)
                        .or_insert_with(Vec::new)
                        .push(p.position.z);
                }
                crate::logging::LogData::Motor(m) => {
                    let motor_value_key = format!("{}/{}/{}", key, "motor", "value").to_string();
                    floats
                        .entry(motor_value_key)
                        .or_insert_with(Vec::new)
                        .push(m.current_value);
                }
                crate::logging::LogData::String(_) => {}
            }
        }
        floats
    }
}

impl DockableWidget for LogGraphWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext) {
        let sim_state = context.state;

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
                        let telem = state.logs;
                        // Limit
                        ui.add(egui::Slider::new(&mut self.limit, 100..=5000).text("Limit"));

                        let mut floats_all: HashMap<String, Vec<f32>> = HashMap::new();

                        for key in telem.keys() {
                            let spawned_entries = self.get_floats_for_entries(telem[key].clone());
                            for (k, v) in spawned_entries {
                                floats_all.insert(k, v);
                            }
                        }

                        let mut keys: Vec<&String> = floats_all.keys().collect();
                        keys.sort();

                        for name in keys {
                            let mut enabled = self.data.contains_key(name.as_str());
                            ui.checkbox(&mut enabled, name.as_str());
                            // Limit selector (default 1000)

                            if enabled {
                                // If the checkbox was checked, add the line to the plot
                                self.data
                                    .entry(name.clone())
                                    .or_insert(floats_all[name].clone());
                            } else {
                                // If the checkbox was unchecked, remove the line from the plot
                                self.data.remove(name.as_str());
                            }
                        }
                    });
                });
            let mut lines = vec![];
            for (_name, data) in &self.data {
                let mut t = 0;
                let sin: PlotPoints = data
                    .into_iter()
                    .map(|v| {
                        t += 1;
                        [t as f64, *v as f64]
                    })
                    .collect();
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
