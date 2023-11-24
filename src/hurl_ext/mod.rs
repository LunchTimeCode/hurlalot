use poll_promise::Promise;
use serde::{Deserialize, Serialize};


struct Resource {
    /// HTTP response
    response: ehttp::Response,

    /// If set, the response was an image.
    message: String,

    error: ParseResponse

}

#[derive(Serialize, Clone)]
pub struct File {
    content: String,
}


pub async fn parse(content: &str, api: &str) -> Option<String> {
    // make the request
    let raw = ettpRequest::post(api)
        .body(File {
            content: content.to_string(),
        })
        .header("Content-Type", "application/json")
        .send()
        .await
        .ok()?
        .json::<ParseResponse>()
        .await
        .ok()?;

    let res = match raw.error {
        None => "all good".to_string(),
        Some(err) => err.message,
    };
    Some(res)
}

#[derive(Deserialize)]
pub struct ParseResponse {
    pub error: Option<ParseError>,
    pub result: Option<String>,
}

#[derive(Deserialize)]
pub struct ParseError {
    pub pos: HurlPos,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct HurlPos {
    pub line: usize,
    pub column: usize,
}