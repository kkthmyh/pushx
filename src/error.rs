use thiserror::Error;

#[derive(Error, Debug)]
pub enum PushError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Response error: {0}")]
    UnexpectedResponse(String),

    #[error("Others error: {0}")]
    Other(String),
}
