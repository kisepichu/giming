use crate::error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::HOME_URL;
use crate::external::atcoder_requester::AtcoderRequester;
use usecases::service::{
    error::ServiceError,
    online_judge::{GetContestArgs, LoginArgs, OnlineJudge, SubmitArgs},
};

use scraper::{Html, Selector};

pub struct Atcoder<'r, R: AtcoderRequester> {
    requester: &'r R,
}

impl<'r, R: AtcoderRequester> Atcoder<'r, R> {
    pub fn new(requester: &'r R) -> Result<Self, DetailError> {
        Ok(Self { requester })
    }

    pub fn whoami(&self) -> Result<String, DetailError> {
        let res = self.requester.get_home()?;
        let text = res.text()?;
        let html = Html::parse_document(&text);
        let selector = Selector::parse("ul.navbar-right .dropdown:last-child ul li a")?;
        let href = html
            .select(&selector)
            .next()
            .ok_or(DetailError::ParsingElementNotFound)?
            .value()
            .attr("href")
            .ok_or(DetailError::ParsingElementNotFound)?
            .to_string();
        let username = href
            .split("/")
            .last()
            .ok_or(DetailError::ParsingError)?
            .to_string();
        Ok(username)
    }
}

impl<'r, R: AtcoderRequester> OnlineJudge<DetailError> for Atcoder<'r, R> {
    fn login(&self, args: LoginArgs) -> Result<(), Box<ServiceError<DetailError>>> {
        (|| -> Result<(), DetailError> {
            let res = self.requester.login(&args.username, &args.password)?;

            let status = res.status();
            let url = res.url().to_string();
            let text = res.text().unwrap();
            if url.contains(HOME_URL) {
                println!("login success");
                Ok(())
            } else if text.contains("You have already signed in.") {
                println!("login success(already signed in)");
                Ok(())
            } else if text.contains("Username or Password is incorrect.") {
                Err(DetailError::InvalidCredentials)
            } else if !status.is_success() {
                Err(DetailError::UnexpectedStatusCode(status))
            } else {
                eprint!("login failed: ");
                Err(DetailError::UnexpectedResponse)
            }
        })()
        .map_err(|e| Box::new(ServiceError::LoginFailed(e)))
    }
    fn get_contest(&self, _args: GetContestArgs) -> Result<(), Box<ServiceError<DetailError>>> {
        todo!()
    }
    fn submit(&self, _args: SubmitArgs) -> Result<(), Box<ServiceError<DetailError>>> {
        todo!()
    }
}
