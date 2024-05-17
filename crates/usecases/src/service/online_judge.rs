#[derive(Debug)]
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

#[cfg_attr(feature = "mock", automock)]
pub trait OnlineJudge<E> {
    fn login(&self, args: LoginArgs) -> Result<(), E>;
    fn get_contest(&self, args: GetContestArgs) -> Result<(), E>;
    fn submit(&self, args: SubmitArgs) -> Result<(), E>;
}
