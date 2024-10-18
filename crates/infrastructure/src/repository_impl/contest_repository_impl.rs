use std::{fs, path::Path};

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

fn ensure_slash(path: &str) -> String {
    if path.ends_with('/') {
        path.to_string()
    } else {
        path.to_string() + "/"
    }
}

impl WorkspaceRepositoryImpl {
    fn generate_dir(
        &self,
        template_path: &str,
        dist_path: &str,
        tera_context: &tera::Context,
    ) -> Result<(), ServiceError<DetailError>> {
        fs::create_dir(dist_path).map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        for entry in fs::read_dir(template_path)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?
        {
            let entry = entry.map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
            let path = entry.path();
            let pathstr = path.clone().into_os_string().into_string().map_err(|_| {
                ServiceError::InitFailed(DetailError::Custom(
                    "PathBuf to String failed".to_string(),
                ))
            })?;
            let dist_path = dist_path.to_string() + &pathstr;
            if path.is_dir() {
                self.generate_dir(path.to_str().unwrap(), &dist_path, tera_context)?;
            } else {
                self.generate_file(path.to_str().unwrap(), &dist_path, tera_context)?;
            }
        }
        Ok(())
    }
    fn generate_file(
        &self,
        template_path: &str,
        dist_path: &str,
        tera_context: &tera::Context,
    ) -> Result<(), ServiceError<DetailError>> {
        let mut tera = tera::Tera::default();
        tera.add_raw_template("template", template_path)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        let rendered = tera
            .render("template", tera_context)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        fs::write(dist_path, rendered)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        Ok(())
    }
}

impl WorkspaceRepository<DetailError> for WorkspaceRepositoryImpl {
    fn exists(&self, contest_id: &str) -> Result<bool, ServiceError<DetailError>> {
        let path = ensure_slash(&self.config.contest_dir) + contest_id;
        Ok(Path::new(&path).exists())
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
        let path = ensure_slash(&self.config.contest_dir) + contest_id;
        let template_path = ensure_slash(&self.config.contest_dir) + &self.config.template_dir_name;
        self.generate_dir(&template_path, &path, &tera_context)?;
        todo!();
    }
}
