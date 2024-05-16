use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid username or password\n")]
    InvalidCredentials,
    #[error("environment variable error: {0}\n")]
    EnvVar(&'static str),
    #[error("invalid status code: {0}\n")]
    InvalidStatusCode(reqwest::StatusCode),
    #[error("reqwest error: {0}\n")]
    Reqwest(#[from] reqwest::Error),
    #[error("scraper error: {0}\n")]
    HtmlParse(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("element not found\n")]
    ParsingElementNotFound,
    #[error("parsing error\n")]
    ParsingError,
    #[error("unknown error\n")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
