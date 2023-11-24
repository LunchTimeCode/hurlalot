use eframe::egui;

use self::{
    highlighter::{highlight, CodeTheme},
    parser::Parser,
};

mod highlighter;
mod parser;

pub struct Editor {
    text: String,
    parser: Parser,
    marker: usize,
}

const ABSTRACT: &str = r"# -------------------------------------------------------------------
# This is a third party tool and not in any association with the Hurl team or Orange S.A. .
# Author of the tool: https://github.com/SilenLoc
# Go to www.hurl.dev for the official Hurl documentation
# 
# Keep in mind that the hurl file is sent to a server for parsing and validation
# It runs code written by the Author and the machine is in control of www.Shuttle.rs / AWS.
# -------------------------------------------------------------------

";

impl Default for Editor {
    fn default() -> Self {
        Self {
            text: ABSTRACT.to_owned(),
            parser: Parser::default(),
            marker: usize::default(),
        }
    }
}

impl Editor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.parser.parse(&self.text);
        let theme = CodeTheme::default();

        if let Some(err) = self.parser.try_to_get_err() {
            self.marker = err.pos.line;
        } else {
            self.marker = usize::MAX;
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button("Add example").clicked() {
                self.text = self.text.clone() + EXAMPLE + "\n\n";
            }
            if ui.button("Abstract").clicked() {
                self.text = self.text.clone() + ABSTRACT + "\n\n";
            }
            if ui.button("Clear").clicked() {
                self.text = String::new();
            }
        });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add_space(1.0);

            ui.vertical(|ui| {
                let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                    let layout_job = highlight(ui.ctx(), &theme, string);
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                egui::ScrollArea::vertical()
                    .id_source("some inner 3")
                    .min_scrolled_height(750.0)
                    .max_height(1000.0)
                    .max_width(1000.0)
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
                                    .desired_width(60.0)
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

                if let Err(err) = self.parser.try_to_get_file() {
                    render_error(&err, ui);
                }

                ui.add_space(10.0);
            });
        });
    }
}

pub fn render_error(err: &str, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label(format!("Error: {err}"));
    });
}

const EXAMPLE: &str = r#"# Testing a JSON response with JSONPath
GET https://example.org/api/tests/{{somepath}}

HTTP 200
[Asserts]
jsonpath "$.status" == "RUNNING"    # Check the status code
jsonpath "$.tests" count == 25      # Check the number of items
jsonpath "$.id" matches /\d{4}/     # Check the format of the id"#;
