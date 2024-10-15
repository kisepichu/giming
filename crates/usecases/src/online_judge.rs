use domain::{entity::Problem, error::Error};

use crate::error::ServiceError;

#[mockall::automock]
pub trait OnlineJudge<E: Error + 'static> {
    fn login(&self, username: String, password: String) -> Result<(), ServiceError<E>>;
    fn get_problems(&self, contest_id: String) -> Result<Vec<Problem>, ServiceError<E>>;
    fn submit(&self, solution_id: String) -> Result<(), ServiceError<E>>;
}
