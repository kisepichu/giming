use crate::infrastructure::external::oj_tools_api::OjToolsApi;
use crate::usecases::repository::{InitError, LoginError, Repository};

use super::external::oj_tools_api::{LoginServiceResponse, OjToolsJson};

pub struct RepositoryImpl {
    oj_tools_api: OjToolsApi,
}

impl RepositoryImpl {
    pub fn new() -> Self {
        Self {
            oj_tools_api: OjToolsApi {},
        }
    }
}

impl Repository for RepositoryImpl {
    fn login(&self, username: String, password: String) -> Result<(), LoginError> {
        match self
            .oj_tools_api
            .login_service(username, password, "https://atcoder.jp".to_string())
        {
            OjToolsJson {
                status: _,
                messages: _,
                result: LoginServiceResponse { logged_in: true },
            } => Ok(()),
            OjToolsJson {
                status,
                messages: _,
                result: LoginServiceResponse { logged_in: false },
            } if status == "forbidden" => Err(LoginError::InvalidCredentials),
            _ => Err(LoginError::Unknown),
        }
    }
    fn init(&self, _contest_id: String) -> Result<(), InitError> {
        // TODO
        Ok(())
    }
}
