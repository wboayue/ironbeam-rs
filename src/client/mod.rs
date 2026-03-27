mod config;
mod http;
pub(crate) mod rest;

pub use config::{ClientBuilder, Credentials};

use bytes::Bytes;
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use http::HttpClient;

/// Authenticated Ironbeam API client.
///
/// Constructed via [`Client::new()`] → [`ClientBuilder`] → [`ClientBuilder::connect()`].
///
/// # Example
///
/// ```no_run
/// # use ironbeam_rs::client::{Client, Credentials};
/// # async fn example() -> ironbeam_rs::error::Result<()> {
/// let client = Client::new()
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
pub struct Client {
    pub(crate) base_url: String,
    pub(crate) token: String,
    pub(crate) auth_headers: HeaderMap,
    pub(crate) http: HttpClient,
}

impl Client {
    /// Start building a new client.
    #[must_use]
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Send an authenticated GET request and deserialize the JSON response.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let uri = format!("{}{path}", self.base_url).parse()?;
        let (status, body) = self.http.get(uri, &self.auth_headers).await?;

        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: String::from_utf8_lossy(&body).into_owned(),
            });
        }

        Ok(serde_json::from_slice(&body)?)
    }

    /// Send an authenticated POST request with a JSON body and deserialize the response.
    #[allow(dead_code)]
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
                message: String::from_utf8_lossy(&resp_body).into_owned(),
            });
        }

        Ok(serde_json::from_slice(&resp_body)?)
    }

}
