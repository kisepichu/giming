use domain::error::Error;

use super::error::ServiceError;

#[derive(Debug)]
pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

pub struct GetContestArgs {
    pub contest_id: String,
}

pub struct SubmitArgs {
    pub solution_id: String,
}

#[cfg_attr(feature = "mock", automock)]
pub trait OnlineJudge<E: Error + 'static> {
    fn login(&self, args: LoginArgs) -> Result<(), Box<ServiceError<E>>>;
    fn get_contest(&self, args: GetContestArgs) -> Result<(), Box<ServiceError<E>>>;
    fn submit(&self, args: SubmitArgs) -> Result<(), Box<ServiceError<E>>>;
}
