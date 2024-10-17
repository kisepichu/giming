use std::fmt;

#[derive(Debug)]
pub enum DetailError {
    EnvVarNotFound(&'static str),
    InvalidInput(String),
    InvalidCredentials(&'static str),
    PermissionDenied(&'static str),
    UnexpectedStatusCode(&'static str, reqwest::StatusCode),
    UnexpectedResponse(&'static str),
    Reqwest(reqwest::Error),
    Scraper(scraper::error::SelectorErrorKind<'static>),
    Confy(confy::ConfyError),
    ParsingElementNotFound(&'static str),
    Parsing(&'static str),
    Internal(&'static str),
    Unknown(anyhow::Error),
}

impl fmt::Display for DetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetailError::EnvVarNotFound(v) => writeln!(f, "environment variable {} not found", v),
            DetailError::InvalidInput(v) => writeln!(f, "invalid input: {}", v),
            DetailError::InvalidCredentials(s) => {
                writeln!(f, "invalid username or password: {}", s)
            }
            DetailError::PermissionDenied(s) => writeln!(f, "permission denied: {}", s),
            DetailError::UnexpectedStatusCode(s, status_code) => {
                writeln!(f, "unexpected status code {}: {}", status_code, s)
            }
            DetailError::UnexpectedResponse(s) => writeln!(f, "unexpected response: {}", s),
            DetailError::Reqwest(err) => writeln!(f, "reqwest error: {}", err),
            DetailError::Scraper(err) => writeln!(f, "scraper error: {}", err),
            DetailError::Confy(err) => writeln!(f, "confy error: {}", err),
            DetailError::ParsingElementNotFound(s) => writeln!(f, "element not found: {}", s),
            DetailError::Parsing(s) => writeln!(f, "parsing error: {}", s),
            DetailError::Internal(s) => writeln!(f, "internal error: {}", s),
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

impl Eq for DetailError {}
