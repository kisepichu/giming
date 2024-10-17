use domain::{entity::Problem, error::Error};

use crate::service_error::ServiceError;

#[mockall::automock]
pub trait DirectoryGenerator<E: Error + 'static> {
    fn generate(&self, contest_id: &str, problems: Vec<Problem>) -> Result<(), ServiceError<E>>;
}
