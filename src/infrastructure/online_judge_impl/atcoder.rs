use crate::domain::error::{Error, Result};
use crate::infrastructure::external::atcoder_requester::atcoder_requester_impl::HOME_URL;
use crate::infrastructure::external::atcoder_requester::AtcoderRequester;
use crate::usecases::service::online_judge::{GetContestArgs, LoginArgs, OnlineJudge, SubmitArgs};

use scraper::{Html, Selector};

pub struct Atcoder<'r, R: AtcoderRequester> {
    requester: &'r R,
}

impl<'r, R: AtcoderRequester> Atcoder<'r, R> {
    pub fn new(requester: &'r R) -> Result<Self> {
        Ok(Self { requester })
    }

    pub fn whoami(&self) -> Result<String> {
        let res = self.requester.get_home()?;
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

impl<'r, R: AtcoderRequester> OnlineJudge for Atcoder<'r, R> {
    fn login(&self, args: LoginArgs) -> Result<()> {
        let res = self.requester.login(&args.username, &args.password)?;

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
