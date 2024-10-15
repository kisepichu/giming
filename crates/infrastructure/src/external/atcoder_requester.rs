use reqwest::blocking::Response;

use crate::error::DetailError;

#[mockall::automock]
pub trait AtcoderRequester {
    fn get_home(&self) -> Result<Response, DetailError>;
    fn login(&self, username: &str, password: &str) -> Result<Response, DetailError>;
    fn get_contest(&self, contest_id: &str) -> Result<Response, DetailError>;
    fn get_tasks(&self, contest_id: &str) -> Result<Response, DetailError>;
    fn get_tasks_print(&self, contest_id: &str) -> Result<Response, DetailError>;
    fn submit(
        &self,
        contest_id: &str,
        problem_id: &str,
        language: usize,
        source: &str,
    ) -> Result<Response, DetailError>;
    // ...
}

pub mod atcoder_requester_impl;
