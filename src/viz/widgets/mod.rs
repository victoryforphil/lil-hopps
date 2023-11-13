use egui::Ui;
use egui_dock::Tree;

use self::dock::{DockableWidget, DockedWindow};

use super::context::VizContext;

pub mod dock;

pub struct WidgetUI {
    dock_tree: Tree<String>,
    last_drock_tree: String,
    dock: DockedWindow,
}

impl WidgetUI {
    pub fn new(viz_context: VizContext) -> Self {
        let mut tree = Tree::new(vec![]);

        Self {
            dock_tree: tree,
            dock: DockedWindow::new("Idk".to_string(), viz_context),
            last_drock_tree: "".to_owned(),
        }
    }

    fn add_dock_widget(&mut self, name: String, widget: Box<dyn DockableWidget>) {
        self.dock.register_window(name.clone(), widget);
        self.dock_tree.push_to_first_leaf(name.clone());
    }

    pub fn draw(&mut self, ctx: &egui::Context, context: &VizContext) {}

    pub fn draw_inside(&mut self, ctx: &egui::Context, ui: &mut Ui, context: &VizContext) {}
}
