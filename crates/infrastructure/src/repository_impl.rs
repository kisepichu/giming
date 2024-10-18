use usecases::repository::{contest_repository::ContestRepository, Repository};

use crate::{config_impl::ConfigImpl, detail_error::DetailError};

use self::contest_repository_impl::ContestRepositoryImpl;

pub mod contest_repository_impl;

pub struct RepositoryImpl {
    contest_repo: ContestRepositoryImpl,
}

impl RepositoryImpl {
    pub fn new(config: &'static ConfigImpl) -> Self {
        Self {
            contest_repo: ContestRepositoryImpl::new(config),
        }
    }
}

impl Repository<DetailError> for RepositoryImpl {
    fn contest_repo(&self) -> &dyn ContestRepository<DetailError> {
        &self.contest_repo
    }
}
