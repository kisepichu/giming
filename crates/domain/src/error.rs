use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error: {0}\n")]
    Unknown(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
