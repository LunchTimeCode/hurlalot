use crate::hurl_ext::{File, HurlPos, ParseError, ParseResponse};
use poll_promise::Promise;
use web_sys::console;

pub struct Parser {
    promise: Option<Promise<ehttp::Result<ParseResponse>>>,
    url: String,

    result: Result<String, ParseError>,

    current_text: String,
    last_text: String,
}

impl Parser {
    pub fn parse(&mut self, text: &str) {
        self.current_text = text.to_string().clone();
        let text_changed = self.last_text.clone() != self.current_text.clone();
        self.last_text = self.current_text.clone();
        if text_changed {
            console::log_1(&"parsing".into());
            self.inner_parse(text);
        }
        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(resource) => {
                        if let Some(error) = resource.error.clone() {
                            self.result = Err(error);
                        } else {
                            self.result = Ok("All good".to_owned());
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
    fn inner_parse(&mut self, text: &str) {
        let (sender, promise) = Promise::new();
        let request = post(&self.url, Into::<File>::into(text.to_string()).into());
        ehttp::fetch(request, move |response| {
            let resource = response.map(ParseResponse::from_response);
            sender.send(resource);
        });
        self.promise = Some(promise);
    }

    pub fn try_to_get_file(&mut self) -> Result<String, String> {
        match &self.result {
            Ok(file) => Ok(file.clone()),
            Err(err) => Err(err.message.clone()),
        }
    }

    pub fn try_to_get_err(&mut self) -> Option<ParseError> {
        self.result.clone().err()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            result: Err(ParseError {
                pos: HurlPos { line: 0, column: 0 },
                message: String::new(),
            }),
            current_text: String::default(),
            last_text: "# Welcome".to_string(),
            promise: Option::default(),
            url: "https://hurlalot.shuttleapp.rs/api/parse".to_owned(),
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn post(url: impl ToString, body: Vec<u8>) -> ehttp::Request {
    ehttp::Request {
        method: "POST".to_owned(),
        url: url.to_string(),
        body,
        headers: ehttp::Headers::new(&[
            ("Accept", "*/*"),
            ("Content-Type", "text/plain; charset=utf-8"),
        ]),
        mode: ehttp::Mode::NoCors,
    }
}
