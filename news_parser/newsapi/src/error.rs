use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fatching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting to string")]
    ConvertingFailed(#[from] std::io::Error),
    #[error("Article parsing failed")]
    ParseFailed(#[from] serde_json::Error),
    #[error("Url parsing failed")]
    UrlParseFailed(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}