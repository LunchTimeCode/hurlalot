use crate::hurl_ext::ext::{
    parse, parse_err_to_pos_err, HurlParseError, HurlParseErrorEnum, HurlPos,
};

pub struct Parser {
    result: Result<String, HurlParseError>,

    text: String,
    last_text: String,

    text_changed: bool,
}

impl Parser {
    pub fn parse(&mut self, text: &str) {
        self.text_changed = self.last_text.eq(&self.text);
        self.last_text = self.text.clone();
        if self.text_changed {
            self.result = parse(text)
        }
    }

    pub fn try_to_get_file(&mut self) -> Result<String, String> {
        match &self.result {
            Ok(file) => Ok(file.clone()),
            Err(err) => Err(parse_err_to_pos_err(&err.inner, err.pos.clone())),
        }
    }

    pub fn try_to_get_err(&mut self) -> Option<HurlParseError> {
        self.result.clone().err()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            result: Err(HurlParseError {
                pos: HurlPos { line: 0, column: 0 },
                inner: HurlParseErrorEnum::Eof {},
            }),
            text: Default::default(),
            text_changed: Default::default(),
            last_text: Default::default(),
        }
    }
}
