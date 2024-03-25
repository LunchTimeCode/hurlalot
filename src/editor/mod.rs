use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme};

use self::parser::Parser;

mod parser;
mod syntax;

pub struct Editor {
    text: String,
    parser: Parser,
    marker: usize,
    syntax: egui_code_editor::Syntax,
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
            syntax: syntax::hurl(),
        }
    }
}
impl Editor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.parser.parse(&self.text);

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
                egui::ScrollArea::vertical()
                    .id_source("some inner 3")
                    .min_scrolled_height(750.0)
                    .max_height(1000.0)
                    .max_width(1000.0)
                    .show(ui, |ui| {
                        ui.push_id("second_some", |ui| {
                            ui.horizontal_top(|ui| {
                                let mut editor = CodeEditor::default()
                                    .id_source("code editor")
                                    .with_rows(10)
                                    .with_fontsize(14.0)
                                    .with_theme(ColorTheme::SONOKAI)
                                    .with_syntax(self.syntax.to_owned())
                                    .with_numlines(true)
                                    .vscroll(true);
                                editor.show(ui, &mut self.text);
                            });
                        });

                        if let Err(err) = self.parser.try_to_get_file() {
                            render_error(&err, ui);
                        }

                        ui.add_space(10.0);
                    });
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
