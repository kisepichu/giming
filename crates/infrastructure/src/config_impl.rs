use confy;
use serde::{Deserialize, Serialize};
use std::env;
use usecases::{config::Config, service_error::ServiceError};

use crate::detail_error::DetailError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigImpl {
    pub solutions_root: String,
    pub prompt: String,
    pub template_dir: String,
    pub template_unstarted_dir: String,
    pub unstarted_marker: String,
    pub problem_file_name: String,
    pub problem_file_template: String,
    pub testcase_in_file: String,
    pub testcase_out_file: String,
    pub testcase_in_template: String,
    pub testcase_out_template: String,
    pub open_command: String,
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            solutions_root: {
                || -> Option<String> {
                    let current_exe=env::current_exe().ok()?;
                    println!("{}", current_exe.to_str()?);
                    let solutions = current_exe.parent()?.parent()?.parent()?.join("solutions");
                    let solutions_str = solutions.to_str()?;
                    if solutions.exists() {
                        Some(solutions_str.to_string())
                    } else {
                        println!("solutions directory not found");
                        None
                    }
                } ().unwrap_or_else(|| {
                    panic!("error occured while initializing ~/.config/giming/config.toml")
                })
            },
            prompt: "{{ contest_id }}> ".to_string(),
            template_dir: "!CONTEST".to_string(),
            template_unstarted_dir: "!CONTEST_UNSTARTED".to_string(),
            unstarted_marker: "!UNSTARTED".to_string(),
            problem_file_name: "!PROBLEM".to_string(),
            problem_file_template: "{{ problem.code | lower }}.rs".to_string(),
            testcase_in_file: "!TESTCASE_IN".to_string(),
            testcase_out_file: "!TESTCASE_OUT".to_string(),
            testcase_in_template: "{{ problem.code | lower }}/in/{{ testcase_index }}.in"
                .to_string(),
            testcase_out_template: "{{ problem.code | lower }}/out/{{ testcase_index }}.out"
                .to_string(),
            open_command: "code {{ contest_dir }}/{{ contest_id }}/{{ contest_id }}.code-workspace --new-window".to_string(),
        }
    }
}

impl ConfigImpl {
    pub fn load() -> Result<Self, ServiceError<DetailError>> {
        confy::load::<ConfigImpl>("giming", "config")
            .map_err(|e| ServiceError::InstantiateFailed(DetailError::from(e)))
    }
}

impl Config for ConfigImpl {}
