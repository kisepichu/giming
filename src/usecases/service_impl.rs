use crate::domain::error::Result;

use super::repository::{LoginArgs, Repository};
use super::service::{InitArgs, Service};

pub struct ServiceImpl<'r, R> {
    repository: &'r R,
}

impl<'r, R> ServiceImpl<'r, R> {
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R: Repository> Service for ServiceImpl<'r, R> {
    fn login(&self, args: LoginArgs) -> Result<()> {
        self.repository.login(args)
    }
    fn init(&self, _args: InitArgs) -> Result<()> {
        // self.repository.get_contest 等使いコンテストディレクトリを作るロジックを書く
        todo!()
    }
}
