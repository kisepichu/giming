use domain::entity::Problem;
use usecases::{repository::contest_repository::ContestRepository, service_error::ServiceError};

use crate::detail_error::DetailError;

#[derive(Default)]
pub struct ContestRepositoryImpl {}

impl ContestRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

// struct SamplePath {
//     input: String,
//     output: String,
// }

impl ContestRepository<DetailError> for ContestRepositoryImpl {
    fn create_if_not_exists(
        &self,
        contest_id: &str,
        _problems: Vec<Problem>,
    ) -> Result<(), ServiceError<DetailError>> {
        // let mut prompt_context = tera::Context::new();
        // prompt_context.insert("contest_id", &self.controller.contest_id());
        // let mut tera = tera::Tera::default();
        // print!(
        //     "{}",
        //     tera.render_str(&self.prompt, &prompt_context).unwrap()
        // );

        // prediction_success, formal_arguments, input_part, sample_paths, test_input_part

        let mut tera_context = tera::Context::new();
        tera_context.insert("contest_id", contest_id);

        todo!();
    }
}
