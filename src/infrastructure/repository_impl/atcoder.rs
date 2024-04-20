use crate::{
    domain,
    usecases::repository::atcoder::{GetProblemError, LoginError, Repository},
};

pub struct AtcoderRepository {
    // スクレイピングのクライアントなど。
}

impl AtcoderRepository {
    pub fn new(/* など */) -> Self {
        Self {}
    }
}

impl Repository for AtcoderRepository {
    fn login(&self, username: String, _password: String) -> Result<(), LoginError> {
        println!("login, username: {}, password: ****", username);
        Ok(())
    }

    fn get_problem(&self, id: String) -> Result<domain::Problem, GetProblemError> {
        println!("get_problem, id: {}", id);
        return Ok(domain::Problem {
            id: "abc998-a".to_string(),
        });
    }
}
