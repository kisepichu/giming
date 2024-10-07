use domain::error::Error;

#[derive(Debug)]
pub enum ServiceError<E: Error + 'static> {
    InstantiateFailed(E),
    LoginFailed(E),
    GetContestFailed(E),
    SubmitFailed(E),
    Unknown,
}

impl<E: Error + 'static> std::fmt::Display for ServiceError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceError::InstantiateFailed(_) => write!(f, "instantiate failed"),
            ServiceError::LoginFailed(_) => write!(f, "login failed"),
            ServiceError::GetContestFailed(_) => write!(f, "get contest failed"),
            ServiceError::SubmitFailed(_) => write!(f, "submit failed"),
            ServiceError::Unknown => write!(f, "unknown error"),
        }
    }
}

impl<E: Error + 'static> std::error::Error for ServiceError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::InstantiateFailed(e) => Some(e),
            ServiceError::LoginFailed(e) => Some(e),
            ServiceError::GetContestFailed(e) => Some(e),
            ServiceError::SubmitFailed(e) => Some(e),
            ServiceError::Unknown => None,
        }
    }
}
