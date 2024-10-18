use domain::error::Error;

use self::contest_repository::ContestRepository;

pub mod contest_repository;

pub trait Repository<E: Error + 'static> {
    fn contest_repo(&self) -> &dyn ContestRepository<E>;
}

pub struct MockRepository<E: Error + 'static> {
    contest_repo: Box<dyn ContestRepository<E>>,
}

impl<E: Error + 'static> MockRepository<E> {
    pub fn new(contest_repo: Box<dyn ContestRepository<E>>) -> Self {
        Self { contest_repo }
    }
}

impl<E: Error + 'static> Repository<E> for MockRepository<E> {
    fn contest_repo(&self) -> &dyn ContestRepository<E> {
        &*self.contest_repo
    }
}
