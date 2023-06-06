use std::collections::BTreeMap;

use eframe::egui;
use egui::Ui;

use super::hurl_ext::parse_err_to_string;

/// We identify tabs by the title of the file we are editing.
type Title = String;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
struct Buffers {
    buffers: BTreeMap<Title, String>,
    current: String,

    extension: String,
    text_changed: bool,
    last_text: String,

    error: String,
}

impl egui_dock::TabViewer for Buffers {
    type Tab = Title;

    fn ui(&mut self, ui: &mut egui::Ui, title: &mut Title) {
        let text = self.buffers.entry(title.clone()).or_default();
        self.text_changed = self.last_text.eq(text);
        self.last_text = text.clone();

        if title.ends_with(".hurl") && self.text_changed {
            match hurl_core::parser::parse_hurl_file(text) {
                Ok(_) => self.error = "none".into(),
                Err(err) => self.error = parse_err_to_string(err.inner, err.pos),
            }
        }

        egui::ScrollArea::vertical()
            .id_source("some inner 3")
            .max_height(500.0)
            .show(ui, |ui| {
                ui.push_id("second_some", |ui| {
                    ui.horizontal_top(|ui| {
                        let mut current: String = text
                            .lines()
                            .take(1000)
                            .enumerate()
                            .map(|(s, _)| (s + 1).to_string() + "\n")
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
                });
            });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(format!("Error: {}", self.error.clone()));
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
