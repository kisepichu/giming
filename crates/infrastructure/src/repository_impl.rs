use usecases::repository::{contest_repository::ContestRepository, Repository};

use crate::detail_error::DetailError;

use self::contest_repository_impl::ContestRepositoryImpl;

pub mod contest_repository_impl;

pub struct RepositoryImpl {
    contest_repo: ContestRepositoryImpl,
}

impl Default for RepositoryImpl {
    fn default() -> Self {
        Self {
            contest_repo: ContestRepositoryImpl::new(),
        }
    }
}

impl Repository<DetailError> for RepositoryImpl {
    fn contest_repo(&self) -> &dyn ContestRepository<DetailError> {
        &self.contest_repo
    }
}
