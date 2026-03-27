use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use hyper::{Response, StatusCode, Uri};
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;

use crate::error::{Error, Result};

type Connector = hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>;

/// Thin HTTP transport layer over hyper + rustls.
///
/// Returns raw `(StatusCode, Bytes)` — callers handle deserialization and
/// error mapping. This keeps the transport composable across REST and streaming.
pub struct HttpClient {
    inner: HyperClient<Connector, Full<Bytes>>,
}

impl HttpClient {
    /// Create a new HTTP client with TLS, connection pooling, and keep-alive.
    ///
    /// Caller must ensure a rustls `CryptoProvider` is installed before calling.
    pub fn new() -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .build();

        let inner = HyperClient::builder(TokioExecutor::new()).build(https);

        Self { inner }
    }

    /// Send a GET request.
    pub async fn get(&self, uri: Uri, headers: &HeaderMap) -> Result<(StatusCode, Bytes)> {
        let mut builder = hyper::Request::builder().method("GET").uri(uri);

        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let req = builder
            .body(Full::new(Bytes::new()))
            .map_err(|e: hyper::http::Error| Error::Other(e.to_string()))?;

        let resp: Response<Incoming> = self.inner.request(req).await?;
        let status = resp.status();
        let body = resp.into_body().collect().await?.to_bytes();

        Ok((status, body))
    }

    /// Send a POST request with a JSON body.
    pub async fn post(
        &self,
        uri: Uri,
        body: Bytes,
        headers: &HeaderMap,
    ) -> Result<(StatusCode, Bytes)> {
        let mut builder = hyper::Request::builder().method("POST").uri(&uri);

        builder = builder.header(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let req = builder
            .body(Full::new(body))
            .map_err(|e: hyper::http::Error| Error::Other(e.to_string()))?;

        let resp: Response<Incoming> = self.inner.request(req).await?;
        let status = resp.status();
        let body = resp.into_body().collect().await?.to_bytes();

        Ok((status, body))
    }
}
