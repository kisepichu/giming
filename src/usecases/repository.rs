use crate::domain::error::Result;

pub struct LoginArgs {
    pub username: String,
    pub password: String,
}

pub struct GetContestArgs {
    pub contest_id: String,
}

pub struct SubmitArgs {
    pub solution_id: String,
}

pub trait Repository {
    fn login(&self, args: LoginArgs) -> Result<()>;
    fn get_contest(&self, args: GetContestArgs) -> Result<()>;
    fn submit(&self, args: SubmitArgs) -> Result<()>;
}
