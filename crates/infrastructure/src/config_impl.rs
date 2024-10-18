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
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            prompt: "{{contest_id}}> ".to_string(),
            contest_dir: "~/repos/giming/solutions".to_string(),
            template_dir_name: "!CONTEST".to_string(),
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
