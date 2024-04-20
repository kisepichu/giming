use thiserror::Error;

use crate::domain;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("login failed with unknown error")]
    Unknown,
}

#[derive(Debug, Error)]
pub enum GetProblemError {
    #[error("problem not found")]
    NotFound,
    #[error("unknown error")]
    Unknown,
}

pub trait Repository {
    fn login(&self, username: String, password: String) -> Result<(), LoginError>;
    fn get_problem(&self, id: String) -> Result<domain::Problem, GetProblemError>;
}
