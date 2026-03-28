use std::future::Future;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use hyper::{Response, StatusCode, Uri};
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;

use crate::error::{Error, Result};

/// Async HTTP transport abstraction.
///
/// Implementors must be cloneable and thread-safe (needed by `Client`'s `Drop`
/// impl which spawns a background logout task via `tokio::spawn`).
pub trait HttpTransport: Clone + Send + Sync + 'static {
    fn get(
        &self,
        uri: Uri,
        headers: &HeaderMap,
    ) -> impl Future<Output = Result<(StatusCode, Bytes)>> + Send;

    fn post(
        &self,
        uri: Uri,
        body: Bytes,
        headers: &HeaderMap,
    ) -> impl Future<Output = Result<(StatusCode, Bytes)>> + Send;

    fn delete(
        &self,
        uri: Uri,
        headers: &HeaderMap,
    ) -> impl Future<Output = Result<(StatusCode, Bytes)>> + Send;
}

type Connector = hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>;

/// Thin HTTP transport layer over hyper + rustls.
///
/// Returns raw `(StatusCode, Bytes)` — callers handle deserialization and
/// error mapping. This keeps the transport composable across REST and streaming.
#[derive(Clone)]
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
}

impl HttpTransport for HttpClient {
    async fn get(&self, uri: Uri, headers: &HeaderMap) -> Result<(StatusCode, Bytes)> {
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

    async fn post(
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

    async fn delete(&self, uri: Uri, headers: &HeaderMap) -> Result<(StatusCode, Bytes)> {
        let mut builder = hyper::Request::builder().method("DELETE").uri(uri);

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
}

#[cfg(test)]
pub(crate) mod mock {
    use std::sync::{Arc, Mutex};

    use bytes::Bytes;
    use hyper::header::HeaderMap;
    use hyper::{StatusCode, Uri};

    use crate::error::Result;

    use super::HttpTransport;

    /// Record of a single HTTP call.
    #[derive(Debug, Clone)]
    pub struct RecordedRequest {
        pub method: &'static str,
        pub uri: Uri,
        pub headers: HeaderMap,
        pub body: Bytes,
    }

    /// Canned response for the mock.
    #[derive(Clone)]
    pub struct MockResponse {
        pub status: StatusCode,
        pub body: Bytes,
    }

    impl MockResponse {
        pub fn ok(body: impl Into<Bytes>) -> Self {
            Self {
                status: StatusCode::OK,
                body: body.into(),
            }
        }

        pub fn error(status: StatusCode, body: impl Into<Bytes>) -> Self {
            Self {
                status,
                body: body.into(),
            }
        }
    }

    /// Test double for [`HttpTransport`].
    ///
    /// Pre-load responses (returned in FIFO order) and inspect recorded requests.
    #[derive(Clone)]
    pub struct MockHttp {
        responses: Arc<Mutex<Vec<MockResponse>>>,
        pub requests: Arc<Mutex<Vec<RecordedRequest>>>,
    }

    impl MockHttp {
        pub fn new(responses: Vec<MockResponse>) -> Self {
            Self {
                responses: Arc::new(Mutex::new(responses)),
                requests: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn recorded_requests(&self) -> Vec<RecordedRequest> {
            self.requests.lock().unwrap().clone()
        }

        fn next_response(&self) -> MockResponse {
            let mut queue = self.responses.lock().unwrap();
            assert!(!queue.is_empty(), "MockHttp: unexpected call — response queue is empty");
            queue.remove(0)
        }
    }

    impl HttpTransport for MockHttp {
        async fn get(&self, uri: Uri, headers: &HeaderMap) -> Result<(StatusCode, Bytes)> {
            self.requests.lock().unwrap().push(RecordedRequest {
                method: "GET",
                uri,
                headers: headers.clone(),
                body: Bytes::new(),
            });
            let r = self.next_response();
            Ok((r.status, r.body))
        }

        async fn post(
            &self,
            uri: Uri,
            body: Bytes,
            headers: &HeaderMap,
        ) -> Result<(StatusCode, Bytes)> {
            self.requests.lock().unwrap().push(RecordedRequest {
                method: "POST",
                uri,
                headers: headers.clone(),
                body,
            });
            let r = self.next_response();
            Ok((r.status, r.body))
        }

        async fn delete(&self, uri: Uri, headers: &HeaderMap) -> Result<(StatusCode, Bytes)> {
            self.requests.lock().unwrap().push(RecordedRequest {
                method: "DELETE",
                uri,
                headers: headers.clone(),
                body: Bytes::new(),
            });
            let r = self.next_response();
            Ok((r.status, r.body))
        }
    }
}
