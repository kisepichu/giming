use domain::error::Error;

use super::error::ServiceError;

// #[derive(Debug)]
// pub struct LoginArgs {
//     pub username: String,
//     pub password: String,
// }

// pub struct GetContestArgs {
//     pub contest_id: String,
// }

// pub struct SubmitArgs {
//     pub solution_id: String,
// }

#[mockall::automock]
pub trait OnlineJudge<E: Error + 'static> {
    fn login(&self, username: String, password: String) -> Result<(), Box<ServiceError<E>>>;
    fn get_contest(&self, contest_id: String) -> Result<(), Box<ServiceError<E>>>;
    fn submit(&self, solution_id: String) -> Result<(), Box<ServiceError<E>>>;
}
