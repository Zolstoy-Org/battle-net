#![forbid(unsafe_code)]

use error::Error;
use result::Result;
use session::Authenticator;
use session::Region;
use session::Session;

pub mod error;
pub mod result;
pub mod session;

pub async fn authenticate(
    client_id: String,
    client_secret: String,
    region: Region,
) -> Result<Session> {
    if client_id.is_empty() || client_secret.is_empty() {
        return Err(Error::InvalidCredentials);
    }

    let session = Authenticator::new()
        .client_id(client_id)
        .client_secret(client_secret)
        .region(region.clone())
        .api_domain(format!("{}.api.blizzard.com", region.api_subdomain()).as_str())
        .auth_domain(format!("{}", region.auth_domain()).as_str())
        .https(true)
        .authenticate()
        .await?;

    Ok(session)
}
