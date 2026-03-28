/// Parsed API error response body.
#[derive(Debug, serde::Deserialize)]
struct ApiErrorBody {
    error1: Option<String>,
    message: Option<String>,
}

/// Extract a human-readable message from an API error JSON body.
/// Prefers `error1` ("Unauthorized"), falls back to `message`, then raw body.
pub(crate) fn parse_api_error(body: &[u8]) -> String {
    if let Ok(parsed) = serde_json::from_slice::<ApiErrorBody>(body) {
        if let Some(e) = parsed.error1.filter(|s| !s.is_empty()) {
            return e;
        }
        if let Some(m) = parsed.message.filter(|s| !s.is_empty()) {
            return m;
        }
    }
    String::from_utf8_lossy(body).into_owned()
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
}
