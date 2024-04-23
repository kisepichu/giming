use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("login failed with unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("init failed with unknown error")]
    Unknown,
}

pub trait Repository {
    fn login(&self, username: String, password: String) -> Result<(), LoginError>;
    fn init(&self, contest_id: String) -> Result<(), InitError>;
}
