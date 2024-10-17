use domain::{
    entity::{Problem, ProblemSummary},
    error::Error,
};

use crate::service_error::ServiceError;

#[mockall::automock]
pub trait OnlineJudge<E: Error + 'static> {
    fn whoami(&self) -> Result<String, ServiceError<E>>;
    fn login(&self, username: String, password: String) -> Result<(), ServiceError<E>>;
    fn get_problems_summary(
        &self,
        contest_id: String,
    ) -> Result<Vec<ProblemSummary>, ServiceError<E>>;
    fn get_problems_detail(&self, contest_id: String) -> Result<Vec<Problem>, ServiceError<E>>;
    fn submit(&self, solution_id: String) -> Result<(), ServiceError<E>>;
}
