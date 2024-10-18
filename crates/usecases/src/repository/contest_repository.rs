use domain::{entity::Problem, error::Error};
use mockall::automock;

use crate::service_error::ServiceError;

#[automock]
pub trait ContestRepository<E: Error + 'static> {
    fn exists(&self, contest_id: &str) -> Result<bool, ServiceError<E>>;
    fn create(&self, contest_id: &str, problems: Vec<Problem>) -> Result<(), ServiceError<E>>;
}
