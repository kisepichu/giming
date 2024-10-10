use confy;
use serde::{Deserialize, Serialize};
use usecases::service::error::ServiceError;

use crate::error::DetailError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub prompt: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prompt: "{{contest_id}}> ".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ServiceError<Box<DetailError>>> {
        confy::load::<Config>("giming", "config")
            .map_err(|e| ServiceError::InstantiateFailed(Box::new(DetailError::from(e))))
    }
}
