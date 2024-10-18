use domain::entity::Workspace;
use usecases::{repository::contest_repository::WorkspaceRepository, service_error::ServiceError};

use crate::{config_impl::ConfigImpl, detail_error::DetailError};

pub struct WorkspaceRepositoryImpl {
    config: &'static ConfigImpl,
}

impl WorkspaceRepositoryImpl {
    pub fn new(config: &'static ConfigImpl) -> Self {
        Self { config }
    }
}

impl WorkspaceRepository<DetailError> for WorkspaceRepositoryImpl {
    fn exists(&self, _contest_id: &str) -> Result<bool, ServiceError<DetailError>> {
        Ok(false) // todo
    }
    fn create(
        &self,
        contest_id: &str,
        workspace: Workspace,
    ) -> Result<(), ServiceError<DetailError>> {
        // let mut prompt_context = tera::Context::new();
        // prompt_context.insert("contest_id", &self.controller.contest_id());
        // let mut tera = tera::Tera::default();
        // print!(
        //     "{}",
        //     tera.render_str(&self.prompt, &prompt_context).unwrap()
        // );

        // problem.{formal_arguments, input_part, sample_paths, test_input_part}

        let mut tera_context = tera::Context::new();
        tera_context.insert("contest_id", contest_id);
        tera_context.insert("problems", &workspace);
        let _path = self.config.contest_dir.clone() + contest_id;
        todo!();
    }
}
