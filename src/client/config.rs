use std::sync::atomic::AtomicBool;

use hyper::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::error::{Error, Result};

use super::http::HttpTransport;
use super::rate_limiter::RateLimiter;
use super::rest::auth;
use super::{Client, RequestHelper, http::HttpClient};

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
    max_requests_per_sec: Option<u32>,
}

impl ClientBuilder {
    pub(crate) fn new() -> Self {
        Self {
            base_url: DEMO_BASE_URL.to_owned(),
            credentials: None,
            max_requests_per_sec: None,
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

    /// Limit outgoing requests to `max_per_sec` per second.
    ///
    /// The Ironbeam API allows 10 requests/sec. Use a value like 8 to stay
    /// safely under the limit.
    #[must_use]
    pub fn rate_limit(mut self, max_per_sec: u32) -> Self {
        self.max_requests_per_sec = Some(max_per_sec);
        self
    }

    /// Authenticate and return a connected [`Client`].
    pub async fn connect(self) -> Result<Client> {
        if rustls::crypto::ring::default_provider()
            .install_default()
            .is_err()
        {
            tracing::debug!("rustls CryptoProvider already installed, using existing");
        }
        let http = HttpClient::new();
        self.connect_with_http(http).await
    }

    /// Authenticate using the provided transport and return a connected [`Client`].
    ///
    /// Extracted from [`connect()`] so tests can inject a mock transport.
    pub(crate) async fn connect_with_http<H: HttpTransport>(
        self,
        http: H,
    ) -> Result<Client<H>> {
        let credentials = self
            .credentials
            .ok_or_else(|| Error::Auth("credentials not set".into()))?;

        let token = auth::authenticate(&http, &self.base_url, &credentials).await?;

        let mut auth_headers = HeaderMap::new();
        let value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| Error::Other(e.to_string()))?;
        auth_headers.insert(AUTHORIZATION, value);

        let rate_limiter = self.max_requests_per_sec.map(RateLimiter::new);

        Ok(Client {
            request: RequestHelper {
                http,
                base_url: self.base_url,
                auth_headers,
                rate_limiter,
            },
            is_logged_out: AtomicBool::new(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::AUTHORIZATION;
    use hyper::StatusCode;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::error::Error;

    use super::*;

    fn builder_with_creds() -> ClientBuilder {
        ClientBuilder::new().credentials(Credentials {
            username: "user".into(),
            password: "pass".into(),
            api_key: "key123".into(),
        })
    }

    #[tokio::test]
    async fn connect_returns_authenticated_client() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok_abc"}"#,
        )]);

        let client = builder_with_creds()
            .connect_with_http(mock)
            .await
            .unwrap();

        let auth = client
            .request
            .auth_headers
            .get(AUTHORIZATION)
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(auth, "Bearer tok_abc");
    }

    #[tokio::test]
    async fn connect_missing_credentials() {
        let mock = MockHttp::new(vec![]);

        let err = ClientBuilder::new()
            .connect_with_http(mock)
            .await
            .unwrap_err();

        assert!(matches!(err, Error::Auth(msg) if msg.contains("credentials")));
    }

    #[tokio::test]
    async fn connect_auth_failure() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::FORBIDDEN,
            r#"{"error1":"Forbidden"}"#,
        )]);

        let err = builder_with_creds()
            .connect_with_http(mock)
            .await
            .unwrap_err();

        assert!(matches!(err, Error::Api { status: 403, .. }));
    }

    #[tokio::test]
    async fn connect_with_rate_limit() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok"}"#,
        )]);

        let client = builder_with_creds()
            .rate_limit(8)
            .connect_with_http(mock)
            .await
            .unwrap();

        assert!(client.request.rate_limiter.is_some());
    }

    #[tokio::test]
    async fn connect_without_rate_limit() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok"}"#,
        )]);

        let client = builder_with_creds()
            .connect_with_http(mock)
            .await
            .unwrap();

        assert!(client.request.rate_limiter.is_none());
    }

    #[test]
    fn credentials_debug_redacts_secrets() {
        let creds = Credentials {
            username: "alice".into(),
            password: "hunter2".into(),
            api_key: "sk-secret".into(),
        };
        let debug = format!("{creds:?}");
        assert!(debug.contains("alice"));
        assert!(!debug.contains("hunter2"));
        assert!(!debug.contains("sk-secret"));
        assert!(debug.contains("***"));
    }

    #[tokio::test]
    async fn connect_demo_sets_demo_url() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok"}"#,
        )]);

        let client = builder_with_creds()
            .demo()
            .connect_with_http(mock)
            .await
            .unwrap();

        assert!(client.request.base_url.contains("demo.ironbeamapi.com"));
    }

    #[tokio::test]
    async fn connect_live_sets_live_url() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok"}"#,
        )]);

        let client = builder_with_creds()
            .live()
            .connect_with_http(mock)
            .await
            .unwrap();

        assert!(client.request.base_url.contains("live.ironbeamapi.com"));
    }

    #[tokio::test]
    async fn connect_uses_custom_base_url() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok"}"#,
        )]);

        let client = builder_with_creds()
            .base_url("http://custom:9090/v2")
            .connect_with_http(mock)
            .await
            .unwrap();

        assert_eq!(client.request.base_url, "http://custom:9090/v2");
        let reqs = client.request.http.recorded_requests();
        assert!(reqs[0].uri.to_string().starts_with("http://custom:9090/v2"));
    }
}
