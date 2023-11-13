

use egui::{ahash::HashMap, Ui};
use egui_dock::{Style, TabViewer};

use crate::viz::context::VizContext;

pub trait DockableWidget {
    fn draw(&mut self, ui: &mut Ui, context: VizContext);
}

pub struct DockedWindow {
    pub title: String,
    pub style: Option<Style>,
    pub current: String,
    pub windows: HashMap<String, Box<dyn DockableWidget>>,
    pub context: VizContext, //TODO: Reference / Pointer
}

impl DockedWindow {
    pub fn new(title: String, init_context: VizContext) -> Self {
        Self {
            title,
            style: None,
            current: "Logs".to_owned(),
            windows: HashMap::default(),
            context: init_context,
        }
    }

    pub fn register_window(&mut self, title: String, widget: Box<dyn DockableWidget>) {
        self.windows.insert(title, widget);
    }

    pub fn update_state(&mut self, state: &VizContext) {
        self.context = state.clone();
    }
}

impl TabViewer for DockedWindow {
    type Tab = String;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match self.windows.get_mut(tab.as_str()) {
            Some(widget) => {
                widget.draw(ui, self.context.clone());
            }
            None => {
                // set window to trasnparent

                ui.label("No widget");
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }
}
