use egui::Ui;
use egui_dock::{DockArea, DockState, Style};

use self::{
    dock::{DockableWidget, DockedWindow},
    runner::RunnerWidget,
};

use super::context::VizContext;

pub mod dock;
pub mod log_graph;
pub mod runner;
pub mod telem_graph;
pub mod three_viz;
pub mod uav_state;
pub mod dataframe_graph;
pub struct WidgetUI {
    dock_tree: DockState<String>,
    last_drock_tree: String,
    dock: DockedWindow,
}

impl WidgetUI {
    pub fn new(viz_context: VizContext) -> Self {
        let tree = DockState::new(vec![]);

        Self {
            dock_tree: tree,
            dock: DockedWindow::new("Idk".to_string(), viz_context),
            last_drock_tree: "".to_owned(),
        }
    }

    pub fn create(&mut self) {
        let runner_widget: Box<_> = Box::new(RunnerWidget::new());
        self.add_dock_widget("Runner".to_string(), runner_widget);

        let uav_state_widget: Box<_> = Box::new(uav_state::UAVStateWidget::new());
        self.add_dock_widget("UAV State".to_string(), uav_state_widget);

        let three_viz_widget: Box<_> = Box::new(three_viz::ThreeVizWidget::new());
        self.add_dock_widget("3D Viz".to_string(), three_viz_widget);

        let telem_graph_widget: Box<_> = Box::new(telem_graph::TelemetryGraphWidget::new());
        self.add_dock_widget("Telemetry Graph".to_string(), telem_graph_widget);

        let log_graph_widget: Box<_> = Box::new(log_graph::LogGraphWidget::new());
        self.add_dock_widget("Log Graph".to_string(), log_graph_widget);
    }

    fn add_dock_widget(&mut self, name: String, widget: Box<dyn DockableWidget>) {
        self.dock.register_window(name.clone(), widget);

        self.dock_tree.push_to_focused_leaf(name.clone());
    }

    pub fn draw(&mut self, ctx: &egui::Context, context: &VizContext) {
        self.dock.update_state(context);
        DockArea::new(&mut self.dock_tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx as &egui_dock::egui::Context, &mut self.dock);
    }

    pub fn draw_inside(&mut self, ctx: &egui::Context, ui: &mut Ui, context: &VizContext) {
        self.dock.update_state(context);
        DockArea::new(&mut self.dock_tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_inside(ui as &mut egui::Ui, &mut self.dock);
    }
}
