#![forbid(unsafe_code)]

use error::Error;
use instance::AuthRegion;
use result::Result;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod instance;
pub mod result;

pub async fn authenticate(
    client_id: &str,
    client_secret: &str,
    auth_region: AuthRegion,
) -> Result<String> {
    let token = authenticate_with_url(client_id, client_secret, auth_region.url()).await?;
    Ok(token)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

pub async fn authenticate_with_url(
    client_id: &str,
    client_secret: &str,
    url: &str,
) -> Result<String> {
    let client = reqwest::Client::new();

    let token = client
        .post(url)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await
        .map_err(|err| Error::HttpError(err))?
        .json::<AuthInfo>()
        .await
        .map_err(|err| Error::HttpError(err))?
        .access_token;
    Ok(token)
}
