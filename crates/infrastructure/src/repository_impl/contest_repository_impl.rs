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

impl WorkspaceRepositoryImpl {
    fn generate_dir(
        &self,
        template_path: &str,
        dest_path: &str,
        tera_context: &tera::Context,
    ) -> Result<(), ServiceError<DetailError>> {
        println!("template_path: {}", template_path);
        println!("dest_path: {}", dest_path);
        println!("---");
        fs::create_dir_all(dest_path).map_err(|e| {
            ServiceError::InitFailed(DetailError::FileSystem(dest_path.to_string(), e))
        })?;
        for template_entry in fs::read_dir(template_path).map_err(|e| {
            ServiceError::InitFailed(DetailError::FileSystem(template_path.to_string(), e))
        })? {
            let template_entry = template_entry.map_err(|e| {
                ServiceError::InitFailed(DetailError::FileSystem(template_path.to_string(), e))
            })?;
            let child_template_pathbuf = template_entry.path();
            let child_template_path = child_template_pathbuf
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|_| {
                    ServiceError::InitFailed(DetailError::Custom(
                        "PathBuf to String failed".to_string(),
                    ))
                })?;
            let child_template_name = child_template_pathbuf
                .file_name()
                .ok_or_else(|| {
                    ServiceError::InitFailed(DetailError::Custom(
                        "Failed to get file name from path".to_string(),
                    ))
                })?
                .to_string_lossy();
            let child_dest_name = {
                let mut tera = tera::Tera::default();
                tera.render_str(&child_template_name, tera_context)
                    .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?
            };
            let child_dest_path = ensure_slash(dest_path) + &child_dest_name;
            if child_template_pathbuf.is_dir() {
                self.generate_dir(&child_template_path, &child_dest_path, tera_context)?;
            } else {
                self.generate_file(&child_template_path, &child_dest_path, tera_context)?;
            }
        }
        Ok(())
    }
    fn generate_file(
        &self,
        template_path: &str,
        dest_path: &str,
        tera_context: &tera::Context,
    ) -> Result<(), ServiceError<DetailError>> {
        let template_content = fs::read_to_string(template_path).map_err(|e| {
            ServiceError::InitFailed(DetailError::FileSystem(template_path.to_string(), e))
        })?;
        let mut tera = tera::Tera::default();
        tera.add_raw_template(template_path, &template_content)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        let rendered = tera
            .render(template_path, tera_context)
            .map_err(|e| ServiceError::InitFailed(DetailError::from(e)))?;
        fs::write(dest_path, rendered).map_err(|e| {
            ServiceError::InitFailed(DetailError::FileSystem(dest_path.to_string(), e))
        })?;
        Ok(())
    }
}

impl WorkspaceRepository<DetailError> for WorkspaceRepositoryImpl {
    fn exists(&self, contest_id: &str) -> Result<bool, ServiceError<DetailError>> {
        let path = expand(&self.config.contest_dir)? + contest_id;
        Path::new(&path)
            .try_exists()
            .map_err(|e| ServiceError::InitFailed(DetailError::FileSystem(path.to_string(), e)))
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
        let path = expand(&self.config.contest_dir)? + contest_id;
        let template_path = expand(&self.config.contest_dir)? + &self.config.template_dir_name;
        self.generate_dir(&template_path, &path, &tera_context)?;
        todo!();
    }
}
