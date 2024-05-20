use crate::error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::HOME_URL;
use crate::external::atcoder_requester::AtcoderRequester;
use usecases::service::{
    error::ServiceError,
    online_judge::{GetContestArgs, LoginArgs, OnlineJudge, SubmitArgs},
};

use scraper::{Html, Selector};

pub struct Atcoder<R: AtcoderRequester> {
    requester: R,
}

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn new(requester: R) -> Self {
        Self { requester }
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

impl<R: AtcoderRequester> OnlineJudge<DetailError> for Atcoder<R> {
    fn login(&self, args: LoginArgs) -> Result<(), Box<ServiceError<DetailError>>> {
        (|| -> Result<(), DetailError> {
            let res = self.requester.login(&args.username, &args.password)?;

            let status = res.status();
            let url = res.url().to_string();
            let text = res.text().unwrap();
            if url.contains(HOME_URL) {
                println!("login success");
                println!("username: {}", self.whoami()?);
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

#[cfg(test)]
mod tests {

    use reqwest::blocking::Response;

    use crate::external::atcoder_requester::MockAtcoderRequester;

    use super::*;

    #[rstest::rstest(path, expected,
        case("tests/responses/atcoder_get_home_logged_in.sanitized.html", Ok("kisepichu".to_string())),
        case("tests/responses/atcoder_get_home_not_logged_in.sanitized.html", Err(DetailError::ParsingElementNotFound)),
    )]
    fn test_whoami(path: &str, expected: Result<String, DetailError>) -> Result<(), String> {
        let requester = MockAtcoderRequester::new();
        let mut atcoder = Atcoder::new(requester);

        let body = std::fs::read_to_string(path).unwrap();
        atcoder
            .requester
            .expect_get_home()
            .times(1)
            .returning(move || Ok(Response::from(http::response::Response::new(body.clone()))));

        let result = atcoder.whoami();
        assert_eq!(result, expected);
        Ok(())
    }
}
