use std::fmt;

#[derive(Debug)]
pub enum DetailError {
    EnvVarNotFound(String),
    InvalidInput(String),
    InvalidCredentials,
    UnexpectedStatusCode(reqwest::StatusCode),
    UnexpectedResponse,
    Reqwest(reqwest::Error),
    Scraper(scraper::error::SelectorErrorKind<'static>),
    ParsingElementNotFound,
    ParsingError,
    Unknown(anyhow::Error),
}

impl fmt::Display for DetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetailError::EnvVarNotFound(v) => write!(f, "environment variable {} not found\n", v),
            DetailError::InvalidInput(v) => write!(f, "invalid input: {}\n", v),
            DetailError::InvalidCredentials => write!(f, "invalid username or password\n"),
            DetailError::UnexpectedStatusCode(status_code) => {
                write!(f, "unexpected status code: {}\n", status_code)
            }
            DetailError::UnexpectedResponse => write!(f, "unexpected response\n"),
            DetailError::Reqwest(err) => write!(f, "reqwest error: {}\n", err),
            DetailError::Scraper(err) => write!(f, "scraper error: {}\n", err),
            DetailError::ParsingElementNotFound => write!(f, "element not found\n"),
            DetailError::ParsingError => write!(f, "parsing error\n"),
            DetailError::Unknown(err) => write!(f, "unknown error: {}\n", err),
        }
    }
}

impl From<reqwest::Error> for DetailError {
    fn from(err: reqwest::Error) -> Self {
        DetailError::Reqwest(err)
    }
}
impl From<scraper::error::SelectorErrorKind<'static>> for DetailError {
    fn from(err: scraper::error::SelectorErrorKind<'static>) -> Self {
        DetailError::Scraper(err)
    }
}
impl From<anyhow::Error> for DetailError {
    fn from(err: anyhow::Error) -> Self {
        DetailError::Unknown(err)
    }
}

impl std::error::Error for DetailError {}

impl PartialEq for DetailError {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
