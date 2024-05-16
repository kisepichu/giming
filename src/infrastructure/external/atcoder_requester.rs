use reqwest::blocking::Response;

use crate::domain::error::Result;

#[cfg_attr(feature = "mock", automock)]
pub trait AtcoderRequester {
    fn get_home(&self) -> Result<Response>;
    fn login(&self, username: &str, password: &str) -> Result<Response>;
    fn get_contest(&self, contest_id: &str) -> Result<Response>;
    fn submit(
        &self,
        contest_id: &str,
        problem_id: &str,
        language: usize,
        source: &str,
    ) -> Result<Response>;
    // ...
}

pub mod atcoder_requester_impl;
