use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("login failed with unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;
