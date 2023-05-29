use egui_dock::{NodeIndex, Style, Tree};

use crate::caller;

use super::{HurlApp, Module};

pub struct Tabs {
    pub tree: Tree<Module>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Tabs {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![Module::Splitter]);
        tree.split_below(
            NodeIndex::root(),
            0.70,
            vec![Module::Connector, Module::Caller, Module::Settings],
        );

        Self { tree }
    }
}

pub fn render(ui: &mut egui::Ui, app: &mut HurlApp) {
    egui_dock::DockArea::new(&mut app.tabs.tree)
        .style(Style::from_egui(ui.style().as_ref()))
        .show_inside(ui, &mut app.tab_view);
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct TabView {
    //states
    caller: caller::req::Client,
}

impl egui_dock::TabViewer for TabView {
    type Tab = Module;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Module::Splitter => {
                ui.label("Splitter");
            }
            Module::Settings => {
                ui.label("Settings");
            }
            Module::Connector => {
                ui.label("Connector");
            }
            Module::Caller => {
                ui.label("Caller");
                self.caller.render(ui)
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let title = match tab {
            Module::Splitter => "Splitter",
            Module::Settings => "Settings",
            Module::Connector => "Connector",
            Module::Caller => "Caller",
        };
        egui::WidgetText::RichText(title.into())
    }
}
