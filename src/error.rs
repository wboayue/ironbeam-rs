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

    #[error("{0}")]
    Other(String),
}

/// Crate-level Result alias.
pub type Result<T> = std::result::Result<T, Error>;
