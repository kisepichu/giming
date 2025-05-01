use std::path::Path;

use domain::entity::Workspace;
use usecases::{repository::contest_repository::WorkspaceRepository, service_error::ServiceError};

use crate::{config_impl::ConfigImpl, detail_error::DetailError};
mod generate;

pub struct WorkspaceRepositoryImpl {
    config: &'static ConfigImpl,
}

impl WorkspaceRepositoryImpl {
    pub fn new(config: &'static ConfigImpl) -> Self {
        Self { config }
    }
}

fn ensure_slash(path: &str) -> String {
    if path.ends_with('/') {
        path.to_string()
    } else {
        path.to_string() + "/"
    }
}

fn expand(path: &str) -> Result<String, ServiceError<DetailError>> {
    Ok(ensure_slash(&path.replace(
        '~',
        &std::env::var("HOME").map_err(|e| {
            ServiceError::InitFailed(DetailError::Custom(format!(
                "Failed to get HOME environment variable: {}",
                e
            )))
        })?,
    )))
}

impl WorkspaceRepository<DetailError> for WorkspaceRepositoryImpl {
    fn exists(&self, contest_id: &str) -> Result<bool, ServiceError<DetailError>> {
        let path = expand(&self.config.contest_dir)? + contest_id;
        Path::new(&path)
            .try_exists()
            .map_err(|e| ServiceError::InitFailed(DetailError::IO(path.to_string(), e)))
    }
    fn create_workspace(
        &self,
        contest_id: &str,
        workspace: &Workspace,
    ) -> Result<(), ServiceError<DetailError>> {
        let dest_path = expand(&self.config.contest_dir)? + contest_id;
        let template_path = expand(&self.config.contest_dir)? + &self.config.template_dir_name;
        self.generate_dir(&template_path, &dest_path, workspace)?;
        Ok(())
    }
    fn get_workspace<'p>(
        &self,
        _contest_id: &str,
    ) -> Result<Workspace<'p>, ServiceError<DetailError>> {
        todo!()
    }
}
