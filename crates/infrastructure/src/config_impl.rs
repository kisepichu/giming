use confy;
use serde::{Deserialize, Serialize};
use usecases::{config::Config, service_error::ServiceError};

use crate::detail_error::DetailError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigImpl {
    pub prompt: String,
    pub contest_dir: String,
    pub template_dir_name: String,
    pub problem_file_name: String,
    pub problem_extrustion: String,
    pub testcases_dir_name: String,
    pub testcase_extrustion_input: String,
    pub testcase_extrustion_output: String,
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            prompt: "{{contest_id}}> ".to_string(),
            contest_dir: "~/repos/giming/solutions".to_string(),
            template_dir_name: "!CONTEST".to_string(),
            problem_file_name: "!PROBLEM".to_string(),
            problem_extrustion: "{{ problem.code | lower }}.rs".to_string(),
            testcases_dir_name: "!TESTCASES".to_string(),
            testcase_extrustion_input: "{{ problem.code }}/in/{{ testcase.index }}.in".to_string(),
            testcase_extrustion_output: "{{ problem.code }}/out/{{ testcase.index }}.out"
                .to_string(),
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
