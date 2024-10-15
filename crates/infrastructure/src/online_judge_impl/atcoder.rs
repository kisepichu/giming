use crate::error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::HOME_URL;
use crate::external::atcoder_requester::AtcoderRequester;

use domain::entity::{Problem, ProblemSummary};
use scraper::{Html, Selector};
use usecases::{error::ServiceError, online_judge::OnlineJudge};

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
            .split('/')
            .last()
            .ok_or(DetailError::ParsingError)?
            .to_string();
        Ok(username)
    }
}

impl<R: AtcoderRequester> OnlineJudge<DetailError> for Atcoder<R> {
    fn login(&self, username: String, password: String) -> Result<(), ServiceError<DetailError>> {
        (|| -> Result<(), DetailError> {
            let res = self.requester.login(&username, &password)?;

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
        .map_err(ServiceError::LoginFailed)
    }
    fn get_problems_summary(
        &self,
        contest_id: String,
    ) -> Result<Vec<ProblemSummary>, ServiceError<DetailError>> {
        || -> Result<Vec<ProblemSummary>, DetailError> {
            let res = self.requester.get_tasks(&contest_id)?;

            let text = res.text()?;
            let html = Html::parse_document(&text);
            let selector = Selector::parse("#main-container>div.row tbody>tr>td:first-child>a")?;
            let elements = html.select(&selector);
            elements
                .map(|e| -> Result<ProblemSummary, DetailError> {
                    let code = e.text().collect::<String>();
                    let url = e
                        .value()
                        .attr("href")
                        .ok_or(DetailError::ParsingElementNotFound)?
                        .to_string();
                    let id = url
                        .split('/')
                        .last()
                        .ok_or(DetailError::ParsingElementNotFound)?
                        .to_string();
                    println!("code: {}, id: {}", code, id);
                    Ok(ProblemSummary { id, code })
                })
                .collect::<Result<Vec<_>, DetailError>>()
        }()
        .map_err(ServiceError::InitFailed)
    }
    fn get_problems_detail(
        &self,
        contest_id: String,
    ) -> Result<Vec<Problem>, ServiceError<DetailError>> {
        let summary = self.get_problems_summary(contest_id.clone())?;
        || -> Result<Vec<Problem>, DetailError> {
            let res = self.requester.get_tasks_print(&contest_id)?;

            let _status = res.status();
            let text = res.text()?;
            let html = Html::parse_document(&text);
            let selector = Selector::parse("#main-container>.row>div:nth-of-type(odd)")?;
            let elements = html.select(&selector);
            let res = elements
                .enumerate()
                .map(|(i, e)| -> Result<Problem, DetailError> {
                    let _id = summary[i].id.clone();
                    let title = {
                        let selector = Selector::parse("span.h2")?;
                        e.select(&selector)
                            .next()
                            .ok_or(DetailError::ParsingElementNotFound)?
                            .text()
                            .collect::<Vec<_>>()
                            .first()
                            .ok_or(DetailError::ParsingElementNotFound)?
                            .to_string() // "A - Problem"
                    };
                    println!("title: {}", title);
                    {
                        let selector = Selector::parse(
                            ":scope>div#task-statement>span>span.lang-en>div:first-of-type",
                        )?;
                        let statement = e
                            .select(&selector)
                            .next()
                            .ok_or(DetailError::ParsingElementNotFound)?
                            .text()
                            .collect::<Vec<_>>()
                            .join("\n");
                        println!("statement: {}", statement);
                    }
                    todo!();
                    // Ok(Problem { title, code, id })
                })
                .collect::<Result<Vec<_>, DetailError>>()?;
            Ok(res)
        }()
        .map_err(ServiceError::InitFailed)
    }
    fn submit(&self, _solution_id: String) -> Result<(), ServiceError<DetailError>> {
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
        case("tests/responses/atcoder_get_home.sanitized.html", Err(DetailError::ParsingElementNotFound)),
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
