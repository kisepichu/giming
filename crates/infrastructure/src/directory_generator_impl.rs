use domain::entity::Problem;
use usecases::{directory_generator::DirectoryGenerator, service_error::ServiceError};

use crate::detail_error::DetailError;

#[derive(Default)]
pub struct DirectoryGeneratorImpl {}

impl DirectoryGeneratorImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl DirectoryGenerator<DetailError> for DirectoryGeneratorImpl {
    fn generate(
        &self,
        _contest_id: &str,
        _problems: Vec<Problem>,
    ) -> Result<(), ServiceError<DetailError>> {
        todo!();
    }
}
