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
    pub problem_file_template: String,
    pub testcase_in_file: String,
    pub testcase_out_file: String,
    pub testcase_in_template: String,
    pub testcase_out_template: String,
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            prompt: "{{ contest_id }}> ".to_string(),
            contest_dir: "~/repos/giming/solutions".to_string(),
            template_dir_name: "!CONTEST".to_string(),
            problem_file_name: "!PROBLEM".to_string(),
            problem_file_template: "{{ problem.code | lower }}.rs".to_string(),
            testcase_in_file: "!TESTCASE_IN".to_string(),
            testcase_out_file: "!TESTCASE_OUT".to_string(),
            testcase_in_template: "{{ problem.code | lower }}/in/{{ testcase_index }}.in"
                .to_string(),
            testcase_out_template: "{{ problem.code | lower }}/out/{{ testcase_index }}.out"
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
