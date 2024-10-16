use std::io::Write;

use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde::Serialize;
use usecases::error::ServiceError;

use crate::error::DetailError;

use super::AtcoderRequester;

pub const BASE_URL: &str = "https://atcoder.jp";
pub const LOGIN_URL: &str = "/login";
pub const HOME_URL: &str = "/home";

pub struct AtcoderRequesterImpl {
    client: Client,
    csrf_token: String,
}

impl AtcoderRequesterImpl {
    pub fn new() -> Result<Self, ServiceError<DetailError>> {
        || -> Result<Self, DetailError> {
            let client = Client::builder().cookie_store(true).build()?;
            let res = client.get(BASE_URL.to_string() + LOGIN_URL).send()?;
            let html = Html::parse_document(&res.text()?);
            let selector = Selector::parse("input[name=csrf_token]")?;
            let csrf_token = html
                .select(&selector)
                .next()
                .ok_or(DetailError::ParsingElementNotFound("new csrf_token"))?
                .value()
                .attr("value")
                .ok_or(DetailError::ParsingElementNotFound("new csrf_token attr"))?
                .to_string();
            Ok(Self { client, csrf_token })
        }()
        .map_err(ServiceError::InstantiateFailed) // |e| ServiceError::InstantiateFailed(e)
    }
}

#[derive(Debug, Serialize)]
struct AtcoderLoginRequest {
    username: String,
    password: String,
    csrf_token: String,
}

const DOWNLOAD: bool = false;

impl AtcoderRequesterImpl {
    fn download_testing_html(&self, url: String, path: &str) -> Result<(), DetailError> {
        if DOWNLOAD {
            let body = self.client.get(url).send()?.text()?;
            let mut file = std::fs::File::create(path).unwrap();
            file.write_all(body.as_bytes()).unwrap();
        }
        Ok(())
    }
}

impl AtcoderRequester for AtcoderRequesterImpl {
    fn get_home(&self) -> Result<Response, DetailError> {
        self.download_testing_html(
            "https://atcoder.jp/home".to_string(),
            "crates/infrastructure/tests/responses/atcoder_get_home_logged_in.html",
        )?;
        Ok(self.client.get(BASE_URL.to_string() + HOME_URL).send()?)
    }
    fn login(&self, username: &str, password: &str) -> Result<Response, DetailError> {
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
    fn get_contest(&self, _contest_id: &str) -> Result<Response, DetailError> {
        todo!()
    }
    fn get_tasks(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}/tasks", contest_id),
            "crates/infrastructure/tests/responses/atcoder_get_tasks_logged_in.html",
        )?;
        Ok(self
            .client
            .get(BASE_URL.to_string() + "/contests/" + contest_id + "/tasks")
            .send()?)
    }
    fn get_tasks_print(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}/tasks_print", contest_id),
            "crates/infrastructure/tests/responses/atcoder_get_tasks_print_logged_in.html",
        )?;
        Ok(self
            .client
            .get(BASE_URL.to_string() + "/contests/" + contest_id + "/tasks_print")
            .send()?)
    }
    fn submit(
        &self,
        _contest_id: &str,
        _problem_id: &str,
        _language: usize,
        _source: &str,
    ) -> Result<Response, DetailError> {
        todo!()
    }
}
