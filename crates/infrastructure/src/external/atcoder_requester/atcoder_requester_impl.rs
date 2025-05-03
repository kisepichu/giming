use std::io::Write;

use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde::Serialize;
use usecases::service_error::ServiceError;

use crate::{config_impl::ConfigImpl, detail_error::DetailError};

use super::AtcoderRequester;

pub const BASE_URL: &str = "https://atcoder.jp";
pub const LOGIN_URL: &str = "/login";
pub const HOME_URL: &str = "/home";

pub struct AtcoderRequesterImpl {
    cookies: String,
}

impl AtcoderRequesterImpl {
    pub fn new(cookies: &str) -> Result<Self, ServiceError<DetailError>> {
        || -> Result<Self, DetailError> {
            // let mut headers = reqwest::header::HeaderMap::new();
            // headers.insert(
            //     reqwest::header::COOKIE,
            //     reqwest::header::HeaderValue::from_str(cookies).unwrap(),
            // );

            Client::builder()
                // .default_headers(headers)
                .cookie_store(true)
                .build()?;
            Ok(Self {
                cookies: cookies.to_string(),
            })
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
    fn download_testing_html(
        &self,
        url: String,
        path: &str,
        force: bool,
    ) -> Result<(), DetailError> {
        if DOWNLOAD || force {
            let client = Client::builder()
                .cookie_store(true)
                .build()
                .map_err(|e| DetailError::Reqwest(e))?;
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
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|e| DetailError::Reqwest(e))?;
        Ok(client.get(BASE_URL.to_string() + HOME_URL).send()?)
    }
    fn login(&self, username: &str, password: &str) -> Result<Response, DetailError> {
        let cookies = {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| DetailError::IO("stdin".to_string(), e))?;
            input.trim().to_string()
        };
        // std::fs::write(&config.atcoder_cookies_path, cookies.clone())
        //     .map_err(|e| DetailError::IO(config.atcoder_cookies_path, e))?;
        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert(
        //     reqwest::header::COOKIE,
        //     reqwest::header::HeaderValue::from_str(&cookies).unwrap(),
        // );
        let client = Client::builder()
            // .default_headers(headers)
            .cookie_store(true)
            .build()?;
        // let form_data = AtcoderLoginRequest {
        //     username: username.to_string(),
        //     password: password.to_string(),
        //     csrf_token: self.csrf_token.clone(),
        // };

        Ok(client
            .post(BASE_URL.to_string() + LOGIN_URL)
            // .form(&form_data)
            .send()?)
    }
    fn get_contest(&self, contest_id: &str) -> Result<Response, DetailError> {
        self.download_testing_html(
            format!("https://atcoder.jp/contests/{}", contest_id),
            "crates/infrastructure/tests/external/atcoder_get_contest.html",
            false,
        )?;
        let client = Client::builder()
            // .default_headers(headers)
            .cookie_store(true)
            .build()?;
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
        let client = Client::builder()
            // .default_headers(headers)
            .cookie_store(true)
            .build()?;
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
        let client = Client::builder()
            // .default_headers(headers)
            .cookie_store(true)
            .build()?;
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
