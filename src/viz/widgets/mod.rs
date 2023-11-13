use egui_dock::Tree;

use self::dock::DockedWindow;

pub mod dock;

pub struct WidgetUI {
    dock_tree: Tree<String>,
    last_drock_tree: String,
    dock: DockedWindow,
}

impl Default for WidgetUI {
    fn default() -> Self {
        let mut tree = Tree::new(vec![]);

        Self {
            dock_tree: tree,
            dock: DockedWindow::new("Idk".to_string()),
            last_drock_tree: "".to_owned(),
        }
    }
}
