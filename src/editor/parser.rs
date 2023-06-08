use hurl_core::{
    ast::{HurlFile, Pos},
    parser::Error,
};

use crate::hurl_ext::parse_err_to_pos_err;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Parser {
    #[serde(skip)]
    file: Option<HurlFile>,
    #[serde(skip)]
    error: Error,
    error_pos: String,
}

impl Parser {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(format!("Error: {}", self.error_pos.clone()));
        });
    }

    pub fn parse(&mut self, text: &str) {
        match hurl_core::parser::parse_hurl_file(text) {
            Ok(file) => {
                self.file = Some(file);
                self.error_pos = "none".into()
            }
            Err(err) => {
                self.error = err.clone();
                self.error_pos = parse_err_to_pos_err(&self.error.inner, err.pos);
            }
        }
    }

}

impl Default for Parser {
    fn default() -> Self {
        Self {
            error: hurl_core::parser::Error {
                pos: Pos { line: 0, column: 0 },
                recoverable: true,
                inner: hurl_core::parser::ParseError::Eof {},
            },
            error_pos: Default::default(),
            file: Default::default(),
        }
    }
}
