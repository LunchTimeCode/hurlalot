use hurl_core::{
    ast::{HurlFile, Pos},
    parser::ParseResult,
};

use crate::hurl_ext::parse_err_to_pos_err;

pub struct Parser {
    result: ParseResult<'static, HurlFile>,

    text: String,
    last_text: String,

    text_changed: bool,
}

impl Parser {
    pub fn parse(&mut self, text: &str) {
        self.text_changed = self.last_text.eq(&self.text);
        self.last_text = self.text.clone();
        if self.text_changed {
            self.result = hurl_core::parser::parse_hurl_file(text)
        }
    }

    pub fn try_to_get_file(&mut self) -> Result<HurlFile, String> {
        match &self.result {
            Ok(file) => Ok(file.clone()),
            Err(err) => Err(parse_err_to_pos_err(&err.inner, err.pos.clone())),
        }
    }

    pub fn try_to_get_err(&mut self) -> Option<hurl_core::parser::Error> {
        self.result.clone().err()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            result: Err(hurl_core::parser::Error {
                pos: Pos { line: 0, column: 0 },
                recoverable: true,
                inner: hurl_core::parser::ParseError::Eof {},
            }),
            text: Default::default(),
            text_changed: Default::default(),
            last_text: Default::default(),
        }
    }
}
