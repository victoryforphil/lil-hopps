use egui::Ui;
use egui_dock::{DockArea, DockState, Style, Tree};

use self::{
    dock::{DockableWidget, DockedWindow},
    runner::RunnerWidget,
};

use super::context::VizContext;

pub mod dock;
pub mod runner;
pub mod uav_state;
pub struct WidgetUI {
    dock_tree: DockState<String>,
    last_drock_tree: String,
    dock: DockedWindow,
}

impl WidgetUI {
    pub fn new(viz_context: VizContext) -> Self {
        let mut tree = DockState::new(vec![]);

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
        self.add_dock_widget("UAV State".to_string(), uav_state_widget)
    }

    fn add_dock_widget(&mut self, name: String, widget: Box<dyn DockableWidget>) {
        self.dock.register_window(name.clone(), widget);
        self.dock_tree.push_to_first_leaf(name.clone());
    }

    pub fn draw(&mut self, ctx: &egui::Context, context: &VizContext) {
        self.dock.update_state(context);
        DockArea::new(&mut self.dock_tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.dock);
    }

    pub fn draw_inside(&mut self, ctx: &egui::Context, ui: &mut Ui, context: &VizContext) {
        self.dock.update_state(context);
        DockArea::new(&mut self.dock_tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_inside(ui, &mut self.dock);
    }
}
