use eframe::egui;
use hurl_core::{ast::Pos, parser::Error};

use crate::hurl_ext::{parse_err_to_message, parse_err_to_pos_err};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Editor {
    current: String,

    text: String,

    extension: String,
    text_changed: bool,
    last_text: String,

    #[serde(skip)]
    error: Error,
    error_pos: String,
    error_message: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            current: Default::default(),
            extension: Default::default(),
            text_changed: Default::default(),
            last_text: Default::default(),
            error: hurl_core::parser::Error {
                pos: Pos { line: 0, column: 0 },
                recoverable: true,
                inner: hurl_core::parser::ParseError::Eof {},
            },
            error_pos: Default::default(),
            error_message: Default::default(),
            text: Default::default(),
        }
    }
}

impl Editor {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.text_changed = self.last_text.eq(&self.text);
        self.last_text = self.text.clone();

        if self.text_changed {
            match hurl_core::parser::parse_hurl_file(&self.text) {
                Ok(_) => self.error_pos = "none".into(),
                Err(err) => {
                    self.error = err.clone();
                    self.error_pos = parse_err_to_pos_err(&self.error.inner, err.pos);
                    self.error_message = parse_err_to_message(&self.error.inner)
                }
            }
        }

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
                            .map(|(s, _)| (s + 1).to_string() + "\n")
                            .collect();

                        egui::TextEdit::multiline(&mut current)
                            .interactive(false)
                            .desired_width(30.0)
                            .code_editor()
                            .show(ui);

                        egui::TextEdit::multiline(&mut self.text)
                            .desired_width(f32::INFINITY)
                            .code_editor()
                            .show(ui);
                    });
                });
            });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(format!("Error: {}", self.error_pos.clone()));
        });
    }
}
