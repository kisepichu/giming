pub mod get_problem;
pub mod login;

use crate::{
    domain,
    usecases::repository::atcoder::{GetProblemError, LoginError, Repository},
};

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

pub struct GetProblemArgs {
    pub problem_id: String,
}

pub trait AtcoderService {
    fn login(&self, args: LoginArgs) -> Result<(), String>;
    fn get_problem(&self, args: GetProblemArgs) -> Result<domain::Problem, String>;
}

pub struct Atcoder<'r, R> {
    repository: &'r R,
}

impl<'r, R> Atcoder<'r, R> {
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R: Repository> Atcoder<'r, R> {
    pub fn login(&self, args: LoginArgs) -> Result<(), LoginError> {
        self.repository.login(args.username, args.password)
    }

    pub fn get_problem(&self, args: GetProblemArgs) -> Result<domain::Problem, GetProblemError> {
        self.repository.get_problem(args.problem_id)
    }
}
