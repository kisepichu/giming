use std::env;
use std::io::Write;

use crate::domain::error::{Error, Result};
use crate::usecases::repository::{GetContestArgs, LoginArgs, Repository, SubmitArgs};

use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::Serialize;

pub struct RepositoryImpl {
    client: Client,
    csrf_token: String,
}

const BASE_URL: &str = "https://atcoder.jp";
const LOGIN_URL: &str = "/login";
const HOME_URL: &str = "/home";

impl RepositoryImpl {
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

    pub fn whoami(&self) -> Result<String> {
        let res = self.client.get(BASE_URL.to_string() + HOME_URL).send()?;
        let text = res.text()?;
        let html = Html::parse_document(&text);
        let selector = Selector::parse("ul.navbar-right .dropdown:last-child ul li a")?;
        let href = html
            .select(&selector)
            .next()
            .ok_or(Error::ParsingElementNotFound)?
            .value()
            .attr("href")
            .ok_or(Error::ParsingElementNotFound)?
            .to_string();
        let username = href
            .split("/")
            .last()
            .ok_or(Error::ParsingError)?
            .to_string();
        Ok(username)
    }
}

#[derive(Debug, Serialize)]
struct AtcoderLoginRequest {
    username: String,
    password: String,
    csrf_token: String,
}

impl Repository for RepositoryImpl {
    fn login(&self, args: LoginArgs) -> Result<()> {
        let username = if args.username.is_empty() {
            env::var("ATCODER_USERNAME").map_err(|_| Error::AtcoderEnvVar("ATCODER_USERNAME"))?
        } else {
            args.username
        };
        let password = if args.password.is_empty() {
            env::var("ATCODER_PASSWORD").map_err(|_| Error::AtcoderEnvVar("ATCODER_PASSWORD"))?
        } else {
            args.password
        };
        let form_data = AtcoderLoginRequest {
            username,
            password,
            csrf_token: self.csrf_token.clone(),
        };
        let res = self
            .client
            .post(BASE_URL.to_string() + LOGIN_URL)
            .form(&form_data)
            .send()
            .unwrap();

        let status = res.status();
        let url = res.url().to_string();
        let text = res.text().unwrap();
        if url.contains(HOME_URL) {
            println!("login success");
            println!("username: {}", self.whoami()?);
            Ok(())
        } else if text.contains("Username or Password is incorrect.") {
            Err(Error::InvalidCredentials)
        } else if !status.is_success() {
            Err(Error::InvalidStatusCode(status))
        } else {
            Err(Error::Unknown)
        }
    }
    fn get_contest(&self, _args: GetContestArgs) -> Result<()> {
        todo!()
    }
    fn submit(&self, _args: SubmitArgs) -> Result<()> {
        todo!()
    }
}
