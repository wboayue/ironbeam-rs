use std::sync::atomic::AtomicBool;

use hyper::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::error::{Error, Result};

use super::rest::auth;
use super::{Client, http::HttpClient};

const DEMO_BASE_URL: &str = "https://demo.ironbeamapi.com/v2";
const LIVE_BASE_URL: &str = "https://live.ironbeamapi.com/v2";

/// Authentication credentials.
#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub api_key: String,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"***")
            .field("api_key", &"***")
            .finish()
    }
}

/// Builder for constructing and connecting a [`Client`].
///
/// Created via [`Client::builder()`]. Configure with fluent methods, then call
/// [`connect()`](ClientBuilder::connect) to authenticate and obtain a `Client`.
pub struct ClientBuilder {
    base_url: String,
    credentials: Option<Credentials>,
}

impl ClientBuilder {
    pub(crate) fn new() -> Self {
        Self {
            base_url: DEMO_BASE_URL.to_owned(),
            credentials: None,
        }
    }

    /// Set authentication credentials.
    #[must_use]
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set a custom base URL.
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Use the demo environment.
    #[must_use]
    pub fn demo(mut self) -> Self {
        self.base_url = DEMO_BASE_URL.to_owned();
        self
    }

    /// Use the live (production) environment.
    #[must_use]
    pub fn live(mut self) -> Self {
        self.base_url = LIVE_BASE_URL.to_owned();
        self
    }

    /// Authenticate and return a connected [`Client`].
    pub async fn connect(self) -> Result<Client> {
        let credentials = self
            .credentials
            .ok_or_else(|| Error::Auth("credentials not set".into()))?;

        if rustls::crypto::ring::default_provider().install_default().is_err() {
            tracing::debug!("rustls CryptoProvider already installed, using existing");
        }
        let http = HttpClient::new();
        let token = auth::authenticate(&http, &self.base_url, &credentials).await?;

        let mut auth_headers = HeaderMap::new();
        let value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| Error::Other(e.to_string()))?;
        auth_headers.insert(AUTHORIZATION, value);

        Ok(Client {
            base_url: self.base_url,
            auth_headers,
            http,
            is_logged_out: AtomicBool::new(false),
        })
    }
}
