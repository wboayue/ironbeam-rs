/// Parsed API error response body.
///
/// Some endpoints (e.g. 429 rate-limit) nest the error inside a `result` object.
#[derive(Debug, serde::Deserialize)]
struct ApiErrorBody {
    error1: Option<String>,
    message: Option<String>,
    result: Option<Box<ApiErrorBody>>,
}

/// Extract a human-readable message from an API error JSON body.
/// Checks top-level `error1`/`message` first, then nested `result`, then raw body.
pub(crate) fn parse_api_error(body: &[u8]) -> String {
    if let Ok(parsed) = serde_json::from_slice::<ApiErrorBody>(body) {
        if let Some(msg) = extract_message(&parsed, 3) {
            return msg;
        }
    }
    String::from_utf8_lossy(body).into_owned()
}

fn extract_message(body: &ApiErrorBody, max_depth: u8) -> Option<String> {
    if let Some(e) = body.error1.as_ref().filter(|s| !s.is_empty()) {
        return Some(e.clone());
    }
    if let Some(m) = body.message.as_ref().filter(|s| !s.is_empty()) {
        return Some(m.clone());
    }
    if max_depth > 0 {
        if let Some(inner) = &body.result {
            return extract_message(inner, max_depth - 1);
        }
    }
    None
}

/// Crate-level error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http: {0}")]
    Http(#[from] hyper::Error),

    #[error("http: {0}")]
    HttpClient(#[from] hyper_util::client::legacy::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("api error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("auth failed: {0}")]
    Auth(String),

    #[error("invalid uri: {0}")]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),

    #[error("websocket: {0}")]
    WebSocket(String),

    #[error("{0}")]
    Other(String),
}

/// Crate-level Result alias.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_api_error_prefers_error1() {
        let body = br#"{"error1":"Unauthorized","message":"bad creds"}"#;
        assert_eq!(parse_api_error(body), "Unauthorized");
    }

    #[test]
    fn parse_api_error_falls_back_to_message() {
        let body = br#"{"message":"something went wrong"}"#;
        assert_eq!(parse_api_error(body), "something went wrong");
    }

    #[test]
    fn parse_api_error_skips_empty_error1() {
        let body = br#"{"error1":"","message":"fallback"}"#;
        assert_eq!(parse_api_error(body), "fallback");
    }

    #[test]
    fn parse_api_error_raw_body_on_invalid_json() {
        let body = b"not json at all";
        assert_eq!(parse_api_error(body), "not json at all");
    }

    #[test]
    fn parse_api_error_raw_body_when_no_fields() {
        let body = br#"{"other":"field"}"#;
        assert_eq!(parse_api_error(body), r#"{"other":"field"}"#);
    }

    #[test]
    fn parse_api_error_nested_result() {
        let body = br#"{"result":{"additionalProperties":{},"error1":"Excessive calls in the last second - maximum allowed is 10","status":1,"message":"Error"},"statusCode":429,"headers":{}}"#;
        assert_eq!(
            parse_api_error(body),
            "Excessive calls in the last second - maximum allowed is 10"
        );
    }
}
