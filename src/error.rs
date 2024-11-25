use thiserror::Error;
use url;

#[derive(Error, Debug)]
pub enum BotifactoryError {
    #[error("Url parsing error")]
    UrlParseError(#[from] url::ParseError),
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("URl Path Error")]
    URLPathError,
    #[error("Invalid identifier")]
    InvalidIdentifier,
}

pub type Result<T, E = BotifactoryError> = std::result::Result<T, E>;