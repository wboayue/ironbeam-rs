use std::sync::atomic::Ordering;

use bytes::Bytes;
use hyper::Method;
use hyper::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::client::Client;
use crate::client::config::Credentials;
use crate::client::http::HttpTransport;
use crate::error::{Error, Result, parse_api_error};
use crate::types::{AuthorizationRequest, AuthorizationResponse, ResponseStatus, SuccessResponse};

/// Authenticate with the Ironbeam API and return the bearer token.
pub async fn authenticate(
    http: &impl HttpTransport,
    base_url: &str,
    credentials: &Credentials,
) -> Result<String> {
    let request = AuthorizationRequest {
        username: credentials.username.clone(),
        password: credentials.password.clone(),
        api_key: credentials.api_key.clone(),
    };

    let uri = format!("{base_url}/auth").parse()?;
    let body = Bytes::from(serde_json::to_vec(&request)?);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let (status, resp_bytes) = http.send(Method::POST, uri, Some(body), &headers).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: parse_api_error(&resp_bytes),
        });
    }

    let resp: AuthorizationResponse = serde_json::from_slice(&resp_bytes)?;

    if resp.status != ResponseStatus::Ok {
        return Err(Error::Auth(
            resp.message.unwrap_or_else(|| "unknown error".into()),
        ));
    }

    let token = resp
        .token
        .ok_or_else(|| Error::Auth("no token in response".into()))?;

    tracing::info!("authenticated successfully");
    Ok(token)
}

/// Invalidate the bearer token.
pub async fn logout<H: HttpTransport>(request: &crate::client::RequestHelper<H>) -> Result<()> {
    let resp: SuccessResponse = request.post("/logout", &serde_json::json!({})).await?;

    if resp.status != ResponseStatus::Ok {
        return Err(Error::Auth(
            resp.message.unwrap_or_else(|| "logout failed".into()),
        ));
    }

    tracing::info!("logged out");
    Ok(())
}

impl<H: HttpTransport> Client<H> {
    /// Log out and invalidate the current bearer token.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// client.logout().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn logout(&self) -> Result<()> {
        logout(&self.request).await?;
        self.is_logged_out.store(true, Ordering::Release);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;

    use hyper::StatusCode;
    use hyper::header::HeaderMap;

    use crate::client::Client;
    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client;
    use crate::error::Error;

    use crate::client::config::Credentials;

    use super::*;

    fn test_credentials() -> Credentials {
        Credentials {
            username: "user".into(),
            password: "pass".into(),
            api_key: "key123".into(),
        }
    }

    #[tokio::test]
    async fn logout_sets_flag() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let client = test_client(mock);

        client.logout().await.unwrap();

        assert!(client.is_logged_out.load(Ordering::Acquire));
        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, hyper::Method::POST);
        assert!(reqs[0].uri.to_string().contains("/logout"));
        assert_eq!(
            reqs[0].headers.get(hyper::header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn logout_propagates_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::UNAUTHORIZED,
            r#"{"error1":"Unauthorized"}"#,
        )]);
        let client = test_client(mock);

        let err = client.logout().await.unwrap_err();
        assert!(matches!(err, Error::Api { status: 401, .. }));
        assert!(!client.is_logged_out.load(Ordering::Acquire));
    }

    #[tokio::test]
    async fn drop_skips_when_already_logged_out() {
        let mock = MockHttp::new(vec![]); // empty queue — would panic if called
        let requests = mock.requests.clone();

        let client = Client {
            request: crate::client::RequestHelper {
                base_url: "http://test".into(),
                auth_headers: HeaderMap::new(),
                http: mock,
            },
            is_logged_out: AtomicBool::new(true),
        };

        drop(client);
        tokio::task::yield_now().await;
        assert!(requests.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn drop_triggers_logout() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let requests = mock.requests.clone();

        let client = test_client(mock);
        drop(client);

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let reqs = requests.lock().unwrap();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, hyper::Method::POST);
        assert!(reqs[0].uri.to_string().contains("/logout"));
    }

    // --- authenticate tests ---

    #[tokio::test]
    async fn authenticate_returns_token() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","token":"tok_abc"}"#,
        )]);

        let token = authenticate(&mock, "http://test", &test_credentials())
            .await
            .unwrap();

        assert_eq!(token, "tok_abc");
        let reqs = mock.recorded_requests();
        assert_eq!(reqs[0].method, hyper::Method::POST);
        assert!(reqs[0].uri.to_string().ends_with("/auth"));
    }

    #[tokio::test]
    async fn authenticate_sends_credentials_in_body() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK","token":"t"}"#)]);

        authenticate(&mock, "http://test", &test_credentials())
            .await
            .unwrap();

        let reqs = mock.recorded_requests();
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["username"], "user");
        assert_eq!(body["password"], "pass");
        assert_eq!(body["apiKey"], "key123");
    }

    #[tokio::test]
    async fn authenticate_http_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::FORBIDDEN,
            r#"{"error1":"Forbidden"}"#,
        )]);

        let err = authenticate(&mock, "http://test", &test_credentials())
            .await
            .unwrap_err();

        assert!(matches!(err, Error::Api { status: 403, .. }));
    }

    #[tokio::test]
    async fn authenticate_error_status_in_body() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"ERROR","message":"invalid credentials"}"#,
        )]);

        let err = authenticate(&mock, "http://test", &test_credentials())
            .await
            .unwrap_err();

        match err {
            Error::Auth(msg) => assert_eq!(msg, "invalid credentials"),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn authenticate_missing_token() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);

        let err = authenticate(&mock, "http://test", &test_credentials())
            .await
            .unwrap_err();

        match err {
            Error::Auth(msg) => assert!(msg.contains("no token")),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }
}
