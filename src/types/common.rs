use serde::{Deserialize, Serialize};

use super::ResponseStatus;

/// Generic API response with status and optional message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub status: ResponseStatus,
    #[serde(default)]
    pub message: Option<String>,
}

/// API success response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub status: ResponseStatus,
    #[serde(default)]
    pub message: Option<String>,
}

/// Order error details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderError {
    #[serde(rename = "errorCode", default)]
    pub error_code: Option<i32>,
    #[serde(rename = "errorText", default)]
    pub error_text: Option<String>,
}

/// WebSocket keepalive ping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PingMessage {
    #[serde(default)]
    pub ping: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserialize() {
        let json = r#"{"status":"OK","message":"Success"}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert_eq!(r.status, ResponseStatus::Ok);
        assert_eq!(r.message.as_deref(), Some("Success"));
    }

    #[test]
    fn response_without_message() {
        let json = r#"{"status":"ERROR"}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        assert_eq!(r.status, ResponseStatus::Error);
        assert_eq!(r.message, None);
    }

    #[test]
    fn order_error_deserialize() {
        let json = r#"{"errorCode":100,"errorText":"Invalid order"}"#;
        let e: OrderError = serde_json::from_str(json).unwrap();
        assert_eq!(e.error_code, Some(100));
        assert_eq!(e.error_text.as_deref(), Some("Invalid order"));
    }
}
