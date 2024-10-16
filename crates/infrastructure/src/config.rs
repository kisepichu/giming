use confy;
use serde::{Deserialize, Serialize};
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub prompt: String,
    pub contest_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prompt: "{{contest_id}}> ".to_string(),
            contest_dir: "~/repos/giming/solutions".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ServiceError<DetailError>> {
        confy::load::<Config>("giming", "config")
            .map_err(|e| ServiceError::InstantiateFailed(DetailError::from(e)))
    }
}
