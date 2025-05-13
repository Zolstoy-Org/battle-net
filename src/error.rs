#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error")]
    GenericError,
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
}
