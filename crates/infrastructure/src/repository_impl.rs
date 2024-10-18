use usecases::repository::{contest_repository::WorkspaceRepository, Repository};

use crate::{config_impl::ConfigImpl, detail_error::DetailError};

use self::contest_repository_impl::WorkspaceRepositoryImpl;

pub mod contest_repository_impl;

pub struct RepositoryImpl {
    contest_repo: WorkspaceRepositoryImpl,
}

impl RepositoryImpl {
    pub fn new(config: &'static ConfigImpl) -> Self {
        Self {
            contest_repo: WorkspaceRepositoryImpl::new(config),
        }
    }
}

impl Repository<DetailError> for RepositoryImpl {
    fn contest_repo(&self) -> &dyn WorkspaceRepository<DetailError> {
        &self.contest_repo
    }
}
