use chrono::{Duration, NaiveDateTime};
use scraper::{Html, Selector};
use usecases::service_error::ServiceError;

use super::Atcoder;
use crate::{detail_error::DetailError, external::atcoder_requester::AtcoderRequester};

fn sleep(duration: Duration) -> bool {
    match duration.to_std() {
        Ok(d) => {
            std::thread::sleep(d);
            true
        }
        Err(_) => false,
    }
}
const INTERVAL: Duration = Duration::seconds(10);
const EARLY_OFFSET: Duration = Duration::seconds(2);
const INTERVAL_LIMIT: Duration = Duration::seconds(1);

impl<R: AtcoderRequester> Atcoder<R> {
    fn get_start_time(&self, contest_id: &str) -> Result<NaiveDateTime, DetailError> {
        let res = self.requester.get_contest(contest_id)?;
        let status = res.status();
        let text = res.text()?;

        if !status.is_success() {
            return Err(DetailError::UnexpectedStatusCode(
                "atcoder get_contest",
                status,
            ));
        }

        let html = Html::parse_document(&text);
        let selector = Selector::parse("small.contest-duration>a>time")?;
        let element = html
            .select(&selector)
            .next()
            .ok_or(DetailError::ParsingElementNotFound(
                "wait_for_start start_time",
            ))?;
        let time_string = element.text().collect::<String>().replace("(Sat)", "");
        let time_string = time_string
            .split('+')
            .next()
            .ok_or(DetailError::Parsing("wait_for_start time_string"))?
            .to_string();
        let time_str = time_string.trim();
        NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_e| DetailError::Parsing("wait_for_start start_time"))
    }

    pub fn wait_for_start(&self, contest_id: &str) -> Result<(), ServiceError<DetailError>> {
        use chrono::prelude::*;
        || -> Result<(), DetailError> {
            let start_time = self.get_start_time(contest_id)?;

            sleep(INTERVAL_LIMIT);
            loop {
                let now = Local::now().naive_local();
                let duration = start_time.signed_duration_since(now);

                if duration.num_seconds() <= INTERVAL.num_seconds() {
                    println!(
                        "current time: {}, starts in: {} seconds",
                        now.format("%Y-%m-%d %H:%M:%S"),
                        duration.num_seconds()
                    );
                    if sleep(Duration::seconds(
                        duration.num_seconds() - EARLY_OFFSET.num_seconds(),
                    )) {
                        let now = Local::now().naive_local();
                        let duration = start_time.signed_duration_since(now);
                        println!(
                            "current time: {}, starts in: {} seconds",
                            now.format("%Y-%m-%d %H:%M:%S"),
                            duration.num_seconds()
                        );
                    } else {
                        let now = Local::now().naive_local();
                        println!(
                            "current time: {}, starting",
                            now.format("%Y-%m-%d %H:%M:%S"),
                        );
                    }
                    break;
                }

                println!(
                    "current time: {}, starts in: {} seconds",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    duration.num_seconds()
                );
                sleep(Duration::seconds(
                    duration.num_seconds()
                        - (duration.num_seconds() - 1) / INTERVAL.num_seconds()
                            * INTERVAL.num_seconds(),
                ));
            }

            Ok(())
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
        case("tests/external/atcoder_get_contest.sanitized.html",
            StatusCode::OK,
            "abc376",
            NaiveDateTime::parse_from_str("2024-10-19 21:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
    )]
    fn test_get_start_time(
        path: &str,
        status: StatusCode,
        args_contest_id: &str,
        expected: NaiveDateTime,
    ) -> Result<(), String> {
        let body = std::fs::read_to_string(path).unwrap();
        let mut requester = MockAtcoderRequester::new();
        let mut response = http::response::Response::new(body.clone());
        *response.status_mut() = status;
        requester
            .expect_get_contest()
            .times(1)
            .returning(move |_| Ok(Response::from(response.clone())));

        let atcoder = Atcoder::new(requester);
        let result = atcoder.get_start_time(args_contest_id);
        assert_eq!(result, Ok(expected));
        Ok(())
    }
}
