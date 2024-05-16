use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde::Serialize;

use crate::domain::error::{Error, Result};

use super::AtcoderRequester;

pub const BASE_URL: &str = "https://atcoder.jp";
pub const LOGIN_URL: &str = "/login";
pub const HOME_URL: &str = "/home";

pub struct AtcoderRequesterImpl {
    client: Client,
    csrf_token: String,
}

impl AtcoderRequesterImpl {
    pub fn new() -> Result<Self> {
        let client = Client::builder().cookie_store(true).build()?;
        let res = client.get(BASE_URL.to_string() + LOGIN_URL).send()?;
        let html = Html::parse_document(&res.text()?);
        let selector = Selector::parse("input[name=csrf_token]")?;
        let csrf_token = html
            .select(&selector)
            .next()
            .ok_or(Error::ParsingElementNotFound)?
            .value()
            .attr("value")
            .ok_or(Error::ParsingElementNotFound)?
            .to_string();
        Ok(Self { client, csrf_token })
    }
}

#[derive(Debug, Serialize)]
struct AtcoderLoginRequest {
    username: String,
    password: String,
    csrf_token: String,
}

impl AtcoderRequester for AtcoderRequesterImpl {
    fn get_home(&self) -> Result<Response> {
        Ok(self.client.get(BASE_URL.to_string() + HOME_URL).send()?)
    }
    fn login(&self, username: &str, password: &str) -> Result<Response> {
        let form_data = AtcoderLoginRequest {
            username: username.to_string(),
            password: password.to_string(),
            csrf_token: self.csrf_token.clone(),
        };
        Ok(self
            .client
            .post(BASE_URL.to_string() + LOGIN_URL)
            .form(&form_data)
            .send()?)
    }
    fn get_contest(&self, _contest_id: &str) -> Result<Response> {
        todo!()
    }
    fn submit(
        &self,
        _contest_id: &str,
        _problem_id: &str,
        _language: usize,
        _source: &str,
    ) -> Result<Response> {
        todo!()
    }
}
