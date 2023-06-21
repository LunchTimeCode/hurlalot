use eframe::egui;

use self::{
    highlighter::{highlight, CodeTheme},
    hurl_render::View,
    parser::Parser,
};

mod fmt;
mod highlighter;
mod hurl_render;
mod parser;

#[derive(Default)]
pub struct Editor {
    text: String,
    parser: Parser,
    marker: usize,
}

impl Editor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.parser.parse(&self.text);
        let theme = CodeTheme::default();

        if let Some(err) = self.parser.try_to_get_err() {
            self.marker = err.pos.line
        } else {
            self.marker = usize::MAX
        }

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = highlight(ui.ctx(), &theme, string);
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::vertical()
            .id_source("some inner 3")
            .max_height(500.0)
            .show(ui, |ui| {
                ui.push_id("second_some", |ui| {
                    ui.horizontal_top(|ui| {
                        let mut current: String = self
                            .text
                            .lines()
                            .take(1000)
                            .enumerate()
                            .map(|(s, _)| {
                                (s + 1).to_string()
                                    + {
                                        if s + 1 == self.marker {
                                            " >"
                                        } else {
                                            ""
                                        }
                                    }
                                    + "\n"
                            })
                            .collect();

                        egui::TextEdit::multiline(&mut current)
                            .font(egui::TextStyle::Monospace)
                            .interactive(false)
                            .desired_width(40.0)
                            .code_editor()
                            .font(egui::FontId::monospace(15.0))
                            .show(ui);

                        egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .code_editor()
                            .lock_focus(true)
                            .layouter(&mut layouter)
                            .show(ui);
                    });
                });
            });

        if ui.button("add example").clicked() {
            self.text = self.text.clone() + EXAMPLE + "\n\n"
        }

        ui.add_space(10.0);

        match self.parser.try_to_get_file() {
            Ok(file) => file.render(ui),
            Err(err) => render_error(&err, ui),
        }
    }
}

pub fn render_error(err: &str, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label(format!("Error: {}", err));
    });
}

const EXAMPLE: &str = r#"# Testing a JSON response with JSONPath
GET https://example.org/api/tests/4567

HTTP 200
[Asserts]
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id"#;
