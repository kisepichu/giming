use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("environment variable error: {0}")]
    EnvVar(&'static str),
    #[error("environment variable error: {0}\n\nFor more information, run 'help login'")]
    AtcoderEnvVar(&'static str),
    #[error("invalid status code: {0}")]
    InvalidStatusCode(reqwest::StatusCode),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("scraper error: {0}")]
    HtmlParse(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("element not found")]
    ParsingElementNotFound,
    #[error("parsing error")]
    ParsingError,
    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
