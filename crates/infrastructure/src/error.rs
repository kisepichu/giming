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
    Confy(confy::ConfyError),
    ParsingElementNotFound,
    ParsingError,
    Unknown(anyhow::Error),
}

impl fmt::Display for DetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetailError::EnvVarNotFound(v) => writeln!(f, "environment variable {} not found", v),
            DetailError::InvalidInput(v) => writeln!(f, "invalid input: {}", v),
            DetailError::InvalidCredentials => writeln!(f, "invalid username or password"),
            DetailError::UnexpectedStatusCode(status_code) => {
                writeln!(f, "unexpected status code: {}", status_code)
            }
            DetailError::UnexpectedResponse => writeln!(f, "unexpected response"),
            DetailError::Reqwest(err) => writeln!(f, "reqwest error: {}", err),
            DetailError::Scraper(err) => writeln!(f, "scraper error: {}", err),
            DetailError::Confy(err) => writeln!(f, "confy error: {}", err),
            DetailError::ParsingElementNotFound => writeln!(f, "element not found"),
            DetailError::ParsingError => writeln!(f, "parsing error"),
            DetailError::Unknown(err) => writeln!(f, "unknown error: {}", err),
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
impl From<confy::ConfyError> for DetailError {
    fn from(err: confy::ConfyError) -> Self {
        DetailError::Confy(err)
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
