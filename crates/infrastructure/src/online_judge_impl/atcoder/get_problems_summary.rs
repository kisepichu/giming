use domain::entity::ProblemSummary;
use scraper::{Html, Selector};
use usecases::service_error::ServiceError;

use crate::{detail_error::DetailError, external::atcoder_requester::AtcoderRequester};

use super::Atcoder;

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn get_problems_summary(
        &self,
        contest_id: &str,
    ) -> Result<Vec<ProblemSummary>, ServiceError<DetailError>> {
        || -> Result<Vec<ProblemSummary>, DetailError> {
            let res = self.requester.get_tasks(contest_id)?;

            let status = res.status();
            let text = res.text()?;

            if !status.is_success() {
                if text.contains("Permission denied.") {
                    return Err(DetailError::PermissionDenied("atcoder get_tasks"));
                }
                return Err(DetailError::UnexpectedStatusCode(
                    "atcoder get_tasks",
                    status,
                ));
            }

            let html = Html::parse_document(&text);
            let selector = Selector::parse("#main-container>div.row tbody>tr>td:first-child>a")?;
            let elements = html.select(&selector);
            elements
                .map(|e| -> Result<ProblemSummary, DetailError> {
                    let code = e.text().collect::<String>();
                    let url = e
                        .value()
                        .attr("href")
                        .ok_or(DetailError::ParsingElementNotFound(
                            "get_problems_summary url",
                        ))?
                        .to_string();
                    let id = url
                        .split('/')
                        .next_back()
                        .ok_or(DetailError::ParsingElementNotFound(
                            "get_problems_summary id",
                        ))?
                        .to_string();
                    println!("code: {}, id: {}", code, id);
                    Ok(ProblemSummary { id, code })
                })
                .collect::<Result<Vec<_>, DetailError>>()
        }()
        .map_err(ServiceError::InitFailed)
    }
}

#[cfg(test)]
mod tests {

    use http::StatusCode;
    use reqwest::blocking::Response;

    use crate::external::atcoder_requester::MockAtcoderRequester;

    use super::*;

    #[rstest::rstest(path, status, args_contest_id, expected,
        case("tests/external/atcoder_get_tasks_logged_in.sanitized.html",
            StatusCode::OK,
            "abc375",
            include!("../../../tests/online_judge_impl/atcoder_get_problems_summary_abc375_logged_in.rs")),
        case("tests/external/atcoder_get_tasks_not_started.sanitized.html",
            StatusCode::NOT_FOUND,
            "abc375",
            Err(ServiceError::InitFailed(DetailError::PermissionDenied("atcoder get_tasks"))),)
    )]
    fn test_get_problems_summary(
        path: &str,
        status: StatusCode,
        args_contest_id: &str,
        expected: Result<Vec<ProblemSummary>, ServiceError<DetailError>>,
    ) -> Result<(), String> {
        let body = std::fs::read_to_string(path).unwrap();
        let mut requester = MockAtcoderRequester::new();
        let mut response = http::response::Response::new(body.clone());
        *response.status_mut() = status;
        requester
            .expect_get_tasks()
            .times(1)
            .returning(move |_| Ok(Response::from(response.clone())));

        let atcoder = Atcoder::new(requester);
        let result = atcoder.get_problems_summary(args_contest_id);
        assert_eq!(result, expected);
        Ok(())
    }
}
