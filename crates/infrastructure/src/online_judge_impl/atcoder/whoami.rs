use scraper::{Html, Selector};
use usecases::service_error::ServiceError;

use crate::{detail_error::DetailError, external::atcoder_requester::AtcoderRequester};

use super::Atcoder;

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn whoami(&self) -> Result<String, ServiceError<DetailError>> {
        || -> Result<String, DetailError> {
            let res = self.requester.get_home()?;
            let text = res.text()?;
            let html = Html::parse_document(&text);
            let selector = Selector::parse("ul.navbar-right .dropdown:last-child ul li a")?;
            let href = html
                .select(&selector)
                .next()
                .ok_or(DetailError::ParsingElementNotFound("whoami href"))?
                .value()
                .attr("href")
                .ok_or(DetailError::ParsingElementNotFound("whoami href attr"))?
                .to_string();
            let username = href
                .split('/')
                .next_back()
                .ok_or(DetailError::Parsing("username"))?
                .to_string();
            Ok(username)
        }()
        .map_err(ServiceError::WhoamiFailed)
    }
}

#[cfg(test)]
mod tests {

    use reqwest::blocking::Response;

    use crate::external::atcoder_requester::MockAtcoderRequester;

    use super::*;

    #[rstest::rstest(path, expected,
        case("tests/external/atcoder_get_home_logged_in.sanitized.html", Ok("kisepichu".to_string())),
        case("tests/external/atcoder_get_home.sanitized.html", Err(ServiceError::WhoamiFailed(DetailError::ParsingElementNotFound("whoami href")))),
    )]
    fn test_whoami(
        path: &str,
        expected: Result<String, ServiceError<DetailError>>,
    ) -> Result<(), String> {
        let body = std::fs::read_to_string(path).unwrap();
        let mut requester = MockAtcoderRequester::new();
        requester
            .expect_get_home()
            .times(1)
            .returning(move || Ok(Response::from(http::response::Response::new(body.clone()))));

        let atcoder = Atcoder::new(requester);
        let result = atcoder.whoami();
        assert_eq!(result, expected);
        Ok(())
    }
}
