use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::Result;
use crate::types::AllAccountsResponse;

impl<H: HttpTransport> Client<H> {
    /// Get all accounts for the authenticated trader.
    pub async fn all_accounts(&self) -> Result<Vec<String>> {
        let resp: AllAccountsResponse = self.get("/account/getAllAccounts").await?;
        Ok(resp.accounts)
    }
}

#[cfg(test)]
mod tests {
    use hyper::StatusCode;
    use hyper::header::AUTHORIZATION;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use crate::error::Error;

    #[tokio::test]
    async fn all_accounts_returns_list() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"accounts":["ACC1","ACC2"]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let accounts = client.all_accounts().await.unwrap();

        assert_eq!(accounts, vec!["ACC1", "ACC2"]);
        let reqs = client.http.recorded_requests();
        assert_eq!(reqs[0].method, "GET");
        assert!(reqs[0].uri.to_string().ends_with("/account/getAllAccounts"));
    }

    #[tokio::test]
    async fn all_accounts_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"accounts":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.all_accounts().await.unwrap();

        let reqs = client.http.recorded_requests();
        assert_eq!(reqs[0].headers.get(AUTHORIZATION).unwrap(), "Bearer tok_test");
    }

    #[tokio::test]
    async fn get_maps_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::NOT_FOUND,
            r#"{"error1":"Not Found"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client.all_accounts().await.unwrap_err();

        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn get_maps_malformed_json() {
        let mock = MockHttp::new(vec![MockResponse::ok(b"not json".to_vec())]);
        let client = test_client_with_auth(mock);

        let err = client.all_accounts().await.unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }
}
