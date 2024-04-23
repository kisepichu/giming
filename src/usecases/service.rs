use crate::usecases::repository::{LoginError, Repository};

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

pub struct GetProblemArgs {
    pub problem_id: String,
}

pub struct Service<'r, R> {
    repository: &'r R,
}

impl<'r, R> Service<'r, R> {
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R: Repository> Service<'r, R> {
    pub fn login(&self, args: LoginArgs) -> Result<(), LoginError> {
        self.repository.login(args.username, args.password)
    }
}
