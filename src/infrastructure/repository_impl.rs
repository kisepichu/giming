#[cfg(test)]
use mockall::automock;

use crate::domain::error::{Error, Result};
use crate::infrastructure::external::oj_tools_api::OjToolsApi;
use crate::usecases::repository::{GetContestArgs, LoginArgs, Repository, SubmitArgs};

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
    fn login(&self, args: LoginArgs) -> Result<()> {
        match self.oj_tools_api.login_service(
            args.username,
            args.password,
            "https://atcoder.jp".to_string(),
        ) {
            OjToolsJson {
                status: _,
                messages: _,
                result: LoginServiceResponse { logged_in: true },
            } => Ok(()),
            OjToolsJson {
                status,
                messages: _,
                result: LoginServiceResponse { logged_in: false },
            } if status == "forbidden" => Err(Error::InvalidCredentials),
            _ => Err(Error::Unknown),
        }
    }
    fn get_contest(&self, _args: GetContestArgs) -> Result<()> {
        todo!()
    }
    fn submit(&self, _args: SubmitArgs) -> Result<()> {
        todo!()
    }
}
