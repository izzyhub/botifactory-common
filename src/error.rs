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
    #[error("Error reading file")]
    IOError(#[from] std::io::Error),
    #[error("Bad header value")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Heapless error")]
    HeaplessError,
}

impl From<()> for BotifactoryError {
    fn from(_: ()) -> Self {
        Self::HeaplessError
    }
}

pub type Result<T, E = BotifactoryError> = std::result::Result<T, E>;
