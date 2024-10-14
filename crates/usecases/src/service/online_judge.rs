use domain::error::Error;

use super::error::ServiceError;

#[mockall::automock]
pub trait OnlineJudge<E: Error + 'static> {
    fn login(&self, username: String, password: String) -> Result<(), Box<ServiceError<E>>>;
    fn get_contest(&self, contest_id: String) -> Result<(), Box<ServiceError<E>>>;
    fn submit(&self, solution_id: String) -> Result<(), Box<ServiceError<E>>>;
}
