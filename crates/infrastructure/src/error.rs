use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid username or password\n")]
    InvalidCredentials,
    #[error("unexpected status code: {0}\n")]
    UnexpectedStatusCode(reqwest::StatusCode),
    #[error("unexpected response\n")]
    UnexpectedResponse,
    #[error("reqwest error\n")]
    Reqwest(#[from] reqwest::Error),
    #[error("scraper error\n")]
    Scraper(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("element not found\n")]
    ParsingElementNotFound,
    #[error("parsing error\n")]
    ParsingError,
    #[error("unknown error: {0}\n")]
    Unknown(#[from] anyhow::Error),
}
