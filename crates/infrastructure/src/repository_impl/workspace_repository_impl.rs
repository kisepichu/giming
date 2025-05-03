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
        let contest_dir = expand(&self.config.solutions_root)? + contest_id;
        Ok(Path::new(&contest_dir).exists())
    }
    fn exists_unstarted(&self, contest_id: &str) -> Result<bool, ServiceError<DetailError>> {
        let contest_dir = expand(&self.config.solutions_root)? + contest_id;
        let unstarted_marker = contest_dir.clone() + "/" + &self.config.unstarted_marker;
        Ok(Path::new(&unstarted_marker).exists())
    }
    fn create(
        &self,
        contest_id: &str,
        workspace: &Workspace,
    ) -> Result<(), ServiceError<DetailError>> {
        let template_path = expand(&self.config.solutions_root)? + &self.config.template_dir;
        let dest_path = expand(&self.config.solutions_root)? + contest_id;
        let unstarted_marker = dest_path.clone() + "/" + &self.config.unstarted_marker;
        if Path::new(&unstarted_marker).exists() {
            std::fs::remove_file(&unstarted_marker)
                .map_err(|e| ServiceError::InitFailed(DetailError::IO(unstarted_marker, e)))?;
        }
        self.generate_dir(&template_path, &dest_path, workspace)?;
        Ok(())
    }
    fn create_unstarted(&self, contest_id: &str) -> Result<(), ServiceError<DetailError>> {
        let workspace = Workspace {
            contest_id: contest_id.to_string(),
            work_problems: vec![],
        };
        let template_path =
            expand(&self.config.solutions_root)? + &self.config.template_unstarted_dir;
        let dest_path = expand(&self.config.solutions_root)? + contest_id;
        self.generate_dir(&template_path, &dest_path, &workspace)?;
        Ok(())
    }
}
