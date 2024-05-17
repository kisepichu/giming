use reqwest::blocking::Response;

use crate::error::Error;

#[cfg_attr(feature = "mock", automock)]
pub trait AtcoderRequester {
    fn get_home(&self) -> Result<Response, Error>;
    fn login(&self, username: &str, password: &str) -> Result<Response, Error>;
    fn get_contest(&self, contest_id: &str) -> Result<Response, Error>;
    fn submit(
        &self,
        contest_id: &str,
        problem_id: &str,
        language: usize,
        source: &str,
    ) -> Result<Response, Error>;
    // ...
}

pub mod atcoder_requester_impl;
