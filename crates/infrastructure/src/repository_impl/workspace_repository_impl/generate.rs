use std::{error::Error, fs};

use domain::entity::Workspace;
use usecases::service_error::ServiceError;

use crate::{detail_error::DetailError, repository_impl::workspace_repository_impl::ensure_slash};

use super::WorkspaceRepositoryImpl;

impl WorkspaceRepositoryImpl {
    pub fn generate_dir(
        &self,
        template_path: &str,
        dest_path: &str,
        workspace: &Workspace,
    ) -> Result<(), ServiceError<DetailError>> {
        fs::create_dir_all(dest_path)
            .map_err(|e| ServiceError::InitFailed(DetailError::IO(dest_path.to_string(), e)))?;
        for template_entry in fs::read_dir(template_path)
            .map_err(|e| ServiceError::InitFailed(DetailError::IO(template_path.to_string(), e)))?
        {
            let template_entry = template_entry.map_err(|e| {
                ServiceError::InitFailed(DetailError::IO(template_path.to_string(), e))
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

            if child_template_name == self.config.problem_file_name {
                let child_dest_names = workspace
                    .work_problems
                    .iter()
                    .map(|p| {
                        let mut tera_context = tera::Context::new();
                        tera_context.insert("contest_id", &workspace.contest_id);
                        tera_context.insert("problem", p.problem);
                        tera_context.insert("io_spec", &p.io_spec);
                        let mut tera = tera::Tera::default();
                        tera.render_str(&self.config.problem_extrustion, &tera_context)
                            .map_err(|e| {
                                ServiceError::InitFailed(DetailError::Internal(
                                    format!(
                                        "error in rendering file name {}:",
                                        child_template_name
                                    ),
                                    Box::new(DetailError::Tera(e)),
                                ))
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                for p in workspace.work_problems.iter().zip(child_dest_names.iter()) {
                    let (work_problem, child_dest_name) = p;
                    let child_dest_path = ensure_slash(dest_path) + child_dest_name;
                    let mut tera_context = tera::Context::new();
                    tera_context.insert("contest_id", &workspace.contest_id);
                    tera_context.insert("problem", work_problem.problem);
                    tera_context.insert("io_spec", &work_problem.io_spec);
                    self.generate_file(&child_template_path, &child_dest_path, &tera_context)?;
                }
            } else {
                let mut tera_context = tera::Context::new();
                tera_context.insert("contest_id", &workspace.contest_id);
                tera_context.insert("work_problems", &workspace.work_problems);
                let child_dest_names = {
                    let mut tera = tera::Tera::default();
                    tera.render_str(&child_template_name, &tera_context)
                        .map_err(|e| {
                            ServiceError::InitFailed(DetailError::Internal(
                                format!("error in rendering file name {}:", child_template_name),
                                Box::new(DetailError::Tera(e)),
                            ))
                        })?
                };
                let child_dest_names = child_dest_names
                    .split(';')
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();
                for child_dest_name in child_dest_names.iter() {
                    let child_dest_path = ensure_slash(dest_path) + child_dest_name;
                    tera_context.insert("filename", &child_dest_name);
                    tera_context.insert("filepath", &child_dest_path);
                    if child_template_pathbuf.is_dir() {
                        self.generate_dir(&child_template_path, &child_dest_path, workspace)?;
                    } else {
                        self.generate_file(&child_template_path, &child_dest_path, &tera_context)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn generate_file(
        &self,
        template_path: &str,
        dest_path: &str,
        tera_context: &tera::Context,
    ) -> Result<(), ServiceError<DetailError>> {
        let template_content = fs::read_to_string(template_path)
            .map_err(|e| ServiceError::InitFailed(DetailError::IO(template_path.to_string(), e)))?;
        let mut tera = tera::Tera::default();
        tera.add_raw_template(template_path, &template_content)
            .map_err(|e| ServiceError::InitFailed(DetailError::Tera(e)))?;
        let rendered = tera.render(template_path, tera_context).map_err(|e| {
            if let Some(s) = e.source() {
                if s.to_string().contains("not found in context") {
                    if let Err(e) = ["problem", "io_spec"]
                        .iter()
                        .map(|v| -> Result<(), ServiceError<DetailError>> {
                            if tera_context.get(v).is_none()
                                && s.to_string().contains(&format!("Variable `{}", v))
                            {
                                Err(ServiceError::InitFailed(DetailError::Custom(format!(
                                    r#"tera error: {:?}

  - tip: maybe using in-problem only variable `{}` in non-problem file,
         problem file name is {}, current file name is {}"#,
                                    e, v, self.config.problem_file_name, template_path
                                ))))
                            } else {
                                Ok(())
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()
                    {
                        return e;
                    }
                }
            }

            ServiceError::InitFailed(DetailError::Internal(
                format!("error in rendering file {}:", template_path),
                Box::new(DetailError::Tera(e)),
            ))
        })?;
        fs::write(dest_path, rendered)
            .map_err(|e| ServiceError::InitFailed(DetailError::IO(dest_path.to_string(), e)))?;
        Ok(())
    }
}
