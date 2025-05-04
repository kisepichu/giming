use reqwest::header::HeaderMap;
use std::{cell::RefCell, io::Write};

use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde::Serialize;
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

use super::AtcoderRequester;

pub const BASE_URL: &str = "https://atcoder.jp";
pub const LOGIN_URL: &str = "/login?continue=https%3A%2F%2Fatcoder.jp%2Fhome";
pub const HOME_URL: &str = "/home";

pub struct AtcoderRequesterImpl {
    cookies: RefCell<Option<String>>,
}

impl AtcoderRequesterImpl {
    pub fn new() -> Result<Self, ServiceError<DetailError>> {
        {
            // let client = Client::builder()
            //     // .default_headers(headers)
            //     .cookie_store(true)
            //     .build()?;
            Ok(Self {
                cookies: RefCell::new(None),
            })
        }
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
    fn build_client(&self) -> Result<Client, DetailError> {
        Ok(if let Some(cookies) = self.cookies.borrow().clone() {
            let mut headers = HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(cookies.as_str()).unwrap(),
            );
            Client::builder()
                .default_headers(headers)
                .cookie_store(true)
                .build()
                .map_err(DetailError::Reqwest)?
        } else {
            Client::builder()
                .cookie_store(true)
                .build()
                .map_err(DetailError::Reqwest)?
        })
    }

    fn download_testing_html(
        &self,
        url: String,
        path: &str,
        force: bool,
    ) -> Result<(), DetailError> {
        if DOWNLOAD || force {
            let client = self.build_client()?;
            let sent = client.get(url).send();
            if sent.is_err() {
                println!("sent: {:?}", sent);
            }
            let body = sent?.text()?;
            let mut file =
                std::fs::File::create(path).map_err(|e| DetailError::IO(path.to_string(), e))?;
            file.write_all(body.as_bytes())
                .map_err(|e| DetailError::IO(path.to_string(), e))?;
        }
        Ok(())
    }
}

impl AtcoderRequester for AtcoderRequesterImpl {
    fn get_home(&self) -> Result<Response, DetailError> {
        self.download_testing_html(
            "https://atcoder.jp/home".to_string(),
            "crates/infrastructure/tests/external/atcoder_get_home_in_contest_logged_in.html",
            false,
        )?;
        let client = self.build_client()?;
        Ok(client.get(BASE_URL.to_string() + HOME_URL).send()?)
    }
    fn login(
        &self,
        username: &str,
        password: &str,
        cookies: Option<String>,
    ) -> Result<Response, DetailError> {
        if let Some(cookies) = cookies {
            self.cookies.replace(Some(cookies));
            let client = self.build_client()?;

            Ok(client.get(BASE_URL.to_string() + HOME_URL).send()?)
        } else {
            let client = self.build_client()?;
            let res = client
                .get(BASE_URL.to_string() + LOGIN_URL)
                .send()
                .map_err(DetailError::Reqwest)?;
            let body = res.text().map_err(DetailError::Reqwest)?;
            let document = Html::parse_document(&body);
            let selector = Selector::parse("input[name=csrf_token]").unwrap();
            let csrf_token = document
                .select(&selector)
                .next()
                .ok_or_else(|| DetailError::Custom("csrf_token not found".to_string()))?
                .value()
                .attr("value")
                .ok_or_else(|| DetailError::Custom("csrf_token value not found".to_string()))?
                .to_string();

            let form_data = AtcoderLoginRequest {
                username: username.to_string(),
                password: password.to_string(),
                csrf_token,
            };

            Ok(client
                .post(BASE_URL.to_string() + LOGIN_URL)
                .form(&form_data)
                .send()?)
        }
    }
    fn get_contest(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}", contest_id),
            "crates/infrastructure/tests/external/atcoder_get_contest.html",
            false,
        )?;
        let client = self.build_client()?;
        Ok(client
            .get(BASE_URL.to_string() + "/contests/" + contest_id)
            .send()?)
    }
    fn get_tasks(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}/tasks", contest_id),
            "crates/infrastructure/tests/external/atcoder_get_tasks_in_contest.html",
            false,
        )?;
        let client = self.build_client()?;
        Ok(client
            .get(BASE_URL.to_string() + "/contests/" + contest_id + "/tasks")
            .send()?)
    }
    fn get_tasks_print(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}/tasks_print", contest_id),
            "crates/infrastructure/tests/external/atcoder_get_tasks_print_in_contest.html",
            false,
        )?;
        let client = self.build_client()?;
        Ok(client
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
