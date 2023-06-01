use std::collections::BTreeMap;

use eframe::egui;
use egui::Ui;

/// We identify tabs by the title of the file we are editing.
type Title = String;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
struct Buffers {
    buffers: BTreeMap<Title, String>,
    current: String,
}

impl egui_dock::TabViewer for Buffers {
    type Tab = Title;

    fn ui(&mut self, ui: &mut egui::Ui, title: &mut Title) {
        let text = self.buffers.entry(title.clone()).or_default();
        ui.horizontal_top(|ui| {
            let mut current: String = text
                .lines()
                .enumerate()
                .map(|(s, _)| s.to_string() + "\n")
                .collect();

            egui::TextEdit::multiline(&mut current)
                .interactive(false)
                .desired_width(30.0)
                .code_editor()
                .show(ui);

            egui::TextEdit::multiline(text)
                .desired_width(f32::INFINITY)
                .code_editor()
                .show(ui);
        });
    }

    fn title(&mut self, title: &mut Title) -> egui::WidgetText {
        egui::WidgetText::from(&*title)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub(crate) struct View {
    #[serde(skip)]
    buffers: Buffers,
    #[serde(skip)]
    tree: egui_dock::Tree<String>,
}

//

impl View {
    pub fn render(&mut self, ui: &mut Ui) {
        for title in self.buffers.buffers.keys() {
            let tab_location = self.tree.find_tab(title);
            let is_open = tab_location.is_some();
            if ui.selectable_label(is_open, title).clicked() {
                if let Some((node_index, tab_index)) = tab_location {
                    self.tree.set_active_tab(node_index, tab_index);
                } else {
                    // Open the file for editing:
                    self.tree.push_to_focused_leaf(title.clone());
                }
            }
        }
    }

    pub fn render_buffers(&mut self, ctx: &egui::Context) {
        egui_dock::DockArea::new(&mut self.tree)
            .id("buffers".into())
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.buffers);
    }

    pub fn add_buffer(&mut self, title: String, file: String) {
        self.buffers.buffers.insert(title, file);
    }
}
