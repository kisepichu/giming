use domain::{entity::Problem, error::Error};
use mockall::automock;

use crate::service_error::ServiceError;

#[automock]
pub trait ContestRepository<E: Error + 'static> {
    fn create_if_not_exists(
        &self,
        contest_id: &str,
        problems: Vec<Problem>,
    ) -> Result<(), ServiceError<E>>;
}
