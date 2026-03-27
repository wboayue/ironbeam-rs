use bytes::Bytes;
use hyper::header::HeaderMap;

use crate::client::config::Credentials;
use crate::client::http::HttpClient;
use crate::error::{Error, Result, parse_api_error};
use crate::types::{AuthorizationRequest, AuthorizationResponse, ResponseStatus};

/// Authenticate with the Ironbeam API and return the bearer token.
pub async fn authenticate(
    http: &HttpClient,
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

    let (status, resp_bytes) = http.post(uri, body, &HeaderMap::new()).await?;

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

    resp.token
        .ok_or_else(|| Error::Auth("no token in response".into()))
}
