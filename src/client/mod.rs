mod config;
mod http;
pub(crate) mod rest;
pub mod stream;

pub use config::{ClientBuilder, Credentials};
pub use http::HttpTransport;

use std::sync::atomic::{AtomicBool, Ordering};

use bytes::Bytes;
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::error::{Error, Result, parse_api_error};
use http::HttpClient;

// ---------------------------------------------------------------------------
// RequestHelper — shared HTTP request logic
// ---------------------------------------------------------------------------

/// Authenticated HTTP request helper shared by [`Client`] and
/// [`StreamHandle`](stream::StreamHandle).
#[derive(Clone)]
pub(crate) struct RequestHelper<H: HttpTransport> {
    pub(crate) http: H,
    pub(crate) base_url: String,
    /// Cached authorization headers. Contains the bearer token — do not log.
    pub(crate) auth_headers: HeaderMap,
}

impl<H: HttpTransport> RequestHelper<H> {
    /// Send an authenticated GET request and deserialize the JSON response.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let uri = format!("{}{path}", self.base_url).parse()?;
        let (status, body) = self.http.get(uri, &self.auth_headers).await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: parse_api_error(&body),
            });
        }

        Ok(serde_json::from_slice(&body)?)
    }

    /// Send an authenticated DELETE request and deserialize the JSON response.
    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let uri = format!("{}{path}", self.base_url).parse()?;
        let (status, body) = self.http.delete(uri, &self.auth_headers).await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: parse_api_error(&body),
            });
        }

        Ok(serde_json::from_slice(&body)?)
    }

    /// Send an authenticated POST request with a JSON body and deserialize the response.
    pub(crate) async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let uri = format!("{}{path}", self.base_url).parse()?;
        let body = Bytes::from(serde_json::to_vec(body)?);

        let (status, resp_body) = self.http.post(uri, body, &self.auth_headers).await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: parse_api_error(&resp_body),
            });
        }

        Ok(serde_json::from_slice(&resp_body)?)
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// Authenticated Ironbeam API client.
///
/// Constructed via [`Client::builder()`] → [`ClientBuilder`] → [`ClientBuilder::connect()`].
///
/// # Example
///
/// ```no_run
/// # use ironbeam_rs::client::{Client, Credentials};
/// # async fn example() -> ironbeam_rs::error::Result<()> {
/// let client = Client::builder()
///     .credentials(Credentials {
///         username: "user".into(),
///         password: "pass".into(),
///         api_key: "key".into(),
///     })
///     .demo()
///     .connect()
///     .await?;
///
/// let accounts = client.all_accounts().await?;
/// # Ok(())
/// # }
/// ```
pub struct Client<H: HttpTransport = HttpClient> {
    pub(crate) request: RequestHelper<H>,
    pub(crate) is_logged_out: AtomicBool,
}

impl<H: HttpTransport> std::fmt::Debug for Client<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.request.base_url)
            .field("auth_headers", &"[redacted]")
            .field("is_logged_out", &self.is_logged_out)
            .finish()
    }
}

impl Client {
    /// Start building a new client.
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

impl<H: HttpTransport> Client<H> {
    /// Send an authenticated GET request and deserialize the JSON response.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request.get(path).await
    }

    /// Send an authenticated POST request with a JSON body and deserialize the response.
    #[allow(dead_code)]
    pub(crate) async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request.post(path, body).await
    }
}

/// Best-effort logout on drop. The spawned task may not run if the tokio
/// runtime is shutting down. Prefer calling [`Client::logout()`] explicitly
/// for guaranteed cleanup.
impl<H: HttpTransport> Drop for Client<H> {
    fn drop(&mut self) {
        if self.is_logged_out.load(Ordering::Acquire) {
            return;
        }

        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };

        let req = self.request.clone();

        handle.spawn(async move {
            if let Err(e) = rest::auth::logout(&req.http, &req.base_url, &req.auth_headers).await {
                tracing::warn!("logout on drop failed: {e}");
            }
        });
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::atomic::AtomicBool;

    use hyper::header::{AUTHORIZATION, HeaderMap, HeaderValue};

    use super::{Client, RequestHelper};
    use super::http::mock::MockHttp;

    /// Build a test client with no auth headers.
    pub fn test_client(mock: MockHttp) -> Client<MockHttp> {
        Client {
            request: RequestHelper {
                base_url: "http://test".into(),
                auth_headers: HeaderMap::new(),
                http: mock,
            },
            is_logged_out: AtomicBool::new(false),
        }
    }

    /// Build a test client with a Bearer auth header.
    pub fn test_client_with_auth(mock: MockHttp) -> Client<MockHttp> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer tok_test"));
        Client {
            request: RequestHelper {
                base_url: "http://test".into(),
                auth_headers: headers,
                http: mock,
            },
            is_logged_out: AtomicBool::new(false),
        }
    }

    // Compile-time assertion: Client must be Send + Sync.
    const _: () = {
        fn _assert_send_sync<T: Send + Sync>() {}
        fn _check() {
            _assert_send_sync::<super::Client>();
        }
    };
}
