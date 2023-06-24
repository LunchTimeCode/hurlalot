pub mod format;

pub mod linter;

use hurl_core::ast::HurlFile;

use super::fmt::linter::{check_hurl_file, Error};

#[derive(Default)]
pub struct Formatter {
    result: Vec<Error>,
    text: String,
    last_text: String,
    text_changed: bool,

    error_text: Vec<String>,
}

impl Formatter {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        if !self.result.is_empty() {
            ui.vertical(|ui| {
                ui.heading("Lint errors:");
                for error in self.error_text.iter() {
                    ui.label(error);
                }
            });
        }
    }

    pub fn check(&mut self, hurl_file: &HurlFile) {
        self.text_changed = self.last_text.eq(&self.text);
        self.last_text = self.text.clone();
        if self.text_changed {
            self.result = check_hurl_file(hurl_file);
            self.error_text = self
                .result
                .clone()
                .iter()
                .map(|err| {
                    let message = match err.inner {
                        crate::editor::fmt::linter::LinterError::UnnecessarySpace {} => {
                            "unnecessary space"
                        }
                        crate::editor::fmt::linter::LinterError::UnnecessaryJsonEncoding {} => {
                            "unnecessary json encoding"
                        }
                        crate::editor::fmt::linter::LinterError::OneSpace {} => "one space",
                    };
                    let from = format!(
                        "Li {} Col {}",
                        err.source_info.start.line, err.source_info.start.column
                    );
                    let to = format!(
                        "Li {} Col {}",
                        err.source_info.end.line, err.source_info.end.column
                    );
                    format!("lint error: {} from {} to {}", message, from, to)
                })
                .collect()
        }
    }
}
