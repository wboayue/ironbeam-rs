use bytes::Bytes;
use hyper::header::HeaderMap;

use crate::client::config::Credentials;
use crate::client::http::HttpClient;
use crate::error::{Error, Result};
use crate::types::{AuthorizationRequest, AuthorizationResponse, ResponseStatus};

/// Authenticate with the Ironbeam API and return the bearer token.
pub async fn authenticate(
    http: &HttpClient,
    base_url: &str,
    credentials: &Credentials,
) -> Result<String> {
    let request = match credentials {
        Credentials::ApiKey { username, api_key } => AuthorizationRequest {
            username: username.clone(),
            password: None,
            api_key: Some(api_key.clone()),
        },
        Credentials::Password { username, password } => AuthorizationRequest {
            username: username.clone(),
            password: Some(password.clone()),
            api_key: None,
        },
    };

    let uri = format!("{base_url}/auth").parse()?;
    let body = Bytes::from(serde_json::to_vec(&request)?);

    let (status, resp_bytes) = http.post(uri, body, &HeaderMap::new()).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: String::from_utf8_lossy(&resp_bytes).into_owned(),
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
