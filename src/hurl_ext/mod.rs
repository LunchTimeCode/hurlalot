use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct File {
    content: String,
}

impl From<String> for File {
    fn from(value: String) -> Self {
        File { content: value }
    }
}

impl From<File> for Vec<u8> {
    fn from(val: File) -> Self {
        let string = serde_json::to_string(&val).unwrap();
        string.as_bytes().to_vec()
    }
}

#[derive(Deserialize, Clone)]
pub struct ParseResponse {
    pub error: Option<ParseError>,
    pub result: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ParseError {
    pub pos: HurlPos,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct HurlPos {
    pub line: usize,
    pub column: usize,
}

impl ParseResponse {
    pub fn from_response(response: ehttp::Response) -> Self {
        if response.status > 299 {
            info!("Server not reachable");
            return Self {
                error: None,
                result: Some("Server not reachable".to_owned()),
            };
        }

        let Ok(res) = String::from_utf8(response.bytes) else {
            info!("could not parse from bytes");
            return Self {
                error: None,
                result: Some("could not from utf 8".to_owned()),
            };
        };

        let response: Self = if let Ok(response) = serde_json::from_str(&res) {
            response
        } else {
            info!("could not parse");
            return Self {
                error: None,
                result: Some("could not from json".to_owned()),
            };
        };

        response
    }
}
