use crate::detail_error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::HOME_URL;
use crate::external::atcoder_requester::AtcoderRequester;

use domain::entity::{Problem, ProblemSummary, Sample};
use scraper::{ElementRef, Html, Selector};
use usecases::{online_judge::OnlineJudge, service_error::ServiceError};

pub struct Atcoder<R: AtcoderRequester> {
    requester: R,
}

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn new(requester: R) -> Self {
        Self { requester }
    }

}

use regex::Regex;

fn next_div(element: ElementRef, f: fn(ElementRef) -> bool) -> Option<ElementRef> {
    let mut node = element.next_sibling()?;
    loop {
        if let Some(ne) = ElementRef::wrap(node) {
            if ne.value().name() == "div" && f(ne) {
                return Some(ne);
            }
        }
        node = node.next_sibling()?;
    }
}

impl<R: AtcoderRequester> OnlineJudge<DetailError> for Atcoder<R> {
    fn name(&self) -> &str {
        "AtCoder"
    }
    fn whoami(&self) -> Result<String, ServiceError<DetailError>> {
        || -> Result<String, DetailError>{
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
                .last()
                .ok_or(DetailError::Parsing("username"))?
                .to_string();
            Ok(username)
        }().map_err(ServiceError::InitFailed)
    }
    fn login(&self, username: String, password: String) -> Result<(), ServiceError<DetailError>> {
        || -> Result<(), DetailError> {
            let res = self.requester.login(&username, &password)?;

            let status = res.status();
            let url = res.url().to_string();
            let text = res.text().unwrap();
            if url.contains(HOME_URL) {
                println!("username: {}", self.whoami().map_err(|e| DetailError::Internal("atcoder login", Box::new(e)))?);
                Ok(())
            } else if text.contains("You have already signed in.") {
                println!("already signed in");
                Ok(())
            } else if text.contains("Username or Password is incorrect.") {
                Err(DetailError::InvalidCredentials("atcoder login"))
            } else if !status.is_success() {
                Err(DetailError::UnexpectedStatusCode("atcoder login", status))
            } else {
                Err(DetailError::UnexpectedResponse("atcoder login"))
            }
        }()
        .map_err(ServiceError::LoginFailed)
    }
    fn get_problems_summary(
        &self,
        contest_id: String,
    ) -> Result<Vec<ProblemSummary>, ServiceError<DetailError>> {
        || -> Result<Vec<ProblemSummary>, DetailError> {
            let res = self.requester.get_tasks(&contest_id)?;

            let status = res.status();
            let text = res.text()?;

            if !status.is_success() {
                if text.contains("Permission denied.") {
                    return Err(DetailError::PermissionDenied("atcoder get_tasks"));
                }
                return Err(DetailError::UnexpectedStatusCode("atcoder get_tasks", status)) 
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
                        .last()
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
    fn get_problems_detail(
        &self,
        contest_id: String,
    ) -> Result<Vec<Problem>, ServiceError<DetailError>> {
        let summary = self.get_problems_summary(contest_id.clone())?;
        || -> Result<Vec<Problem>, DetailError> {
            let res = self.requester.get_tasks_print(&contest_id)?;

            let status = res.status();
            let text = res.text()?;

            if !status.is_success() {
                if text.contains("Permission denied.") {
                    return Err(DetailError::PermissionDenied("atcoder get_tasks_print"));
                }
                return Err(DetailError::UnexpectedStatusCode("atcoder get_tasks_print", status)) 
            }

            let html = Html::parse_document(&text);
            let selector = Selector::parse("#main-container>.row>div:nth-of-type(odd)")?;
            let elements = html.select(&selector);
            let res = elements
                .enumerate()
                .map(|(i, e)| -> Result<Problem, DetailError> {
                    let title = {
                        let selector = Selector::parse(":scope>span.h2")?;
                        e.select(&selector)
                            .next()
                            .ok_or(DetailError::ParsingElementNotFound(
                                "get_problems_detail title",
                            ))?
                            .text()
                            .collect::<Vec<_>>()
                            .first()
                            .ok_or(DetailError::ParsingElementNotFound(
                                "get_problems_detail title first",
                            ))?
                            .to_string() // "A - Problem"
                    };
                    println!("title: {}", title);

                    let (time_limit, memory_limit) = {
                        let selector = Selector::parse(":scope>p")?;
                        let limits_vec = e
                            .select(&selector)
                            .next()
                            .ok_or(DetailError::ParsingElementNotFound("get_problems_detail limits_vec"))?
                            .text()
                            .collect::<Vec<_>>();
                        let limits_str = limits_vec
                            .first()
                            .ok_or(DetailError::Parsing("get_problems_detail limits_str"))?;

                        let re = Regex::new(r"Time Limit: ([\d\.]+)(?:\s*(ms|sec)) / Memory Limit: (\d+)\s*MB")
                            .map_err(|_| DetailError::Custom("get_problems_detail regex error".to_string()))?;

                        if let Some(captures) = re.captures(limits_str) {
                            let time_value: f64 = captures[1]
                                .parse()
                                .map_err(|_| DetailError::Parsing("get_problems_detail time_value"))?;
                            let time_limit = match &captures[2] {
                                "ms" => time_value as usize,
                                _ => (time_value * 1000.) as usize,
                            };

                            let memory_limit: usize = captures[3]
                                .parse()
                                .map_err(|_| DetailError::Parsing("get_problems_detail memory_limit"))?;

                            println!("time_limit: {}, memory_limit: {}", time_limit, memory_limit);
                            Ok((time_limit, memory_limit))
                        } else {
                            Err(DetailError::Parsing(
                                "get_problems_detail time_limit line not found or unexpected format",
                            ))
                        }
                    }?;

                    let point: usize;
                    let statement: String;
                    let constraints: Vec<String>;
                    let input_format: String;
                    let mut samples: Vec<Sample> = Vec::new();
                    {
                        // {
                        //     let test = |s: &str| -> Option<ElementRef> {
                        //         let selector = Selector::parse(s).unwrap();
                        //         e.select(&selector).next()
                        //     };
                        //     test(":scope>div").unwrap();
                        //     test(":scope>div#task-statement").unwrap();
                        //     test(":scope>div#task-statement>span").unwrap();
                        //     test(":scope>div#task-statement>span>span.lang-en").unwrap();
                        //     test(":scope>div#task-statement>span>span.lang-en>p").unwrap();
                        // }
                        let selector =
                            Selector::parse(":scope>div#task-statement>span>span.lang-en>p")?;
                        let mut task_e = e
                            .select(&selector)
                            .next()
                            .ok_or(DetailError::ParsingElementNotFound("get_problems_detail task_e point"))?;
                        {
                            let selector = Selector::parse(":scope>var")?;
                            let point_string = task_e
                                .select(&selector)
                                .next()
                                .ok_or(DetailError::ParsingElementNotFound("get_problems_detail point_string"))?
                                .text()
                                .collect::<Vec<_>>()
                                .join("");
                            point = point_string.parse().map_err(|_| DetailError::Parsing("get_problems_detail point"))?;
                        }
                        println!("point: {}", point);

                        // let next_div = |element: ElementRef,
                        //                 f: fn(ElementRef) -> bool|
                        //  -> Option<ElementRef> {
                        //     let mut e = element;
                        //     while let Some(node) = e.next_sibling() {
                        //         if let Some(ne) = ElementRef::wrap(node) {
                        //             if ne.value().name() == "div" && f(ne) {
                        //                 return Some(ne);
                        //             }
                        //             e = ne;
                        //         }
                        //     }
                        //     None
                        // };

                        task_e = next_div(task_e, |_| true).ok_or(DetailError::Parsing("get_problems_detail task_e statement"))?;
                        println!("statement"); 
                        statement = task_e.text().collect::<Vec<_>>().join("\n");
                        println!("statement:\n{}", statement); 

                        task_e = next_div(task_e, |_| true).ok_or(DetailError::Parsing("get_problems_detail task_e constraints"))?;
                        {
                            let selector = Selector::parse(":scope ul>li")?;
                            constraints = task_e
                                .select(&selector)
                                .map(|e| e.text().collect())
                                .collect();
                        }
                        println!("constraints: {:?}", constraints);

                        task_e = next_div(task_e, |e| e.value().attr("class") == Some("io-style"))
                            .ok_or(DetailError::Parsing("get_problems_detail task_e input_format"))?;
                        {
                            let selector = Selector::parse(":scope>div:first-child pre")?;
                            input_format = task_e
                                .select(&selector)
                                .next()
                                .ok_or(DetailError::ParsingElementNotFound("get_problems_detail input_format"))?
                                .text()
                                .collect::<Vec<_>>()
                                .join("\n");
                        }
                        println!("input_format: {}", input_format);

                        loop {
                            task_e = match next_div(task_e, |_| true) {
                                Some(e) => e,
                                None => break,
                            };
                            let selector = Selector::parse(":scope>section>pre")?;
                            let sample_input = task_e
                                .select(&selector)
                                .next()
                                .ok_or(DetailError::ParsingElementNotFound("get_problems_detail sample_input"))?
                                .text()
                                .collect::<Vec<_>>()
                                .join("");

                            task_e = next_div(task_e, |_| true).ok_or(DetailError::Parsing("get_problems_detail task_e sample_output"))?;
                            let sample_output = task_e
                                .select(&selector)
                                .next()
                                .ok_or(DetailError::ParsingElementNotFound("get_problems_detail sample_output"))?
                                .text()
                                .collect::<Vec<_>>()
                                .join("");

                            println!("input:\n{}\noutput:\n{}", sample_input.clone(), sample_output.clone());
                            samples.push(Sample {
                                input: sample_input,
                                output: sample_output,
                            });
                        }
                    }
                    Ok(Problem {
                        title,
                        id: summary[i].id.clone(),
                        code: summary[i].code.clone(),
                        statement,
                        constraints,
                        input_format,
                        samples,
                        point,
                        time_limit,
                        memory_limit,
                    })
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

    use http::StatusCode;
    use reqwest::blocking::Response;

    use crate::external::atcoder_requester::MockAtcoderRequester;

    use super::*;

    #[rstest::rstest(path, expected,
        case("tests/external/atcoder_get_home_logged_in.sanitized.html", Ok("kisepichu".to_string())),
        case("tests/external/atcoder_get_home.sanitized.html", Err(ServiceError::InitFailed(DetailError::ParsingElementNotFound("whoami href")))),
    )]
    fn test_whoami(path: &str, expected: Result<String, ServiceError<DetailError>>) -> Result<(), String> {
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
    #[rstest::rstest(path, status, args_contest_id, expected,
        case("tests/external/atcoder_get_tasks_logged_in.sanitized.html",
            StatusCode::OK,
            "abc375",
            include!("../../tests/online_judge_impl/atcoder_get_problems_summary_abc375_logged_in.txt")),
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
        let result = atcoder.get_problems_summary(args_contest_id.to_string());
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest::rstest(get_tasks_path, get_tasks_status, get_tasks_print_path, get_tasks_print_call, get_tasks_print_status, args_contest_id, expected,
        case("tests/external/atcoder_get_tasks_logged_in.sanitized.html",
            StatusCode::OK, 
            "tests/external/atcoder_get_tasks_print_logged_in.sanitized.html",
            1,
            StatusCode::OK,
            "abc375",
            include!("../../tests/online_judge_impl/atcoder_get_problems_detail_abc375_logged_in.txt")),
        case("tests/external/atcoder_get_tasks_not_started.sanitized.html",
            StatusCode::NOT_FOUND,
            "tests/external/atcoder_get_tasks_print_not_started.sanitized.html",
            0,
            StatusCode::NOT_FOUND,
            "abc376",
            Err(ServiceError::InitFailed(DetailError::PermissionDenied("atcoder get_tasks"))))
    )]
    fn test_get_problems_detail(
        get_tasks_path: &str,
        get_tasks_status: StatusCode,
        get_tasks_print_path: &str,
        get_tasks_print_call: usize,
        get_tasks_print_status: StatusCode,
        args_contest_id: &str,
        expected: Result<Vec<Problem>, ServiceError<DetailError>>,
    ) -> Result<(), String> {
        let mut requester = MockAtcoderRequester::new();
        let body = std::fs::read_to_string(get_tasks_path).unwrap();
        let mut response = http::response::Response::new(body.clone());
        *response.status_mut() = get_tasks_status;
        requester
            .expect_get_tasks()
            .times(1)
            .returning(move |_| Ok(Response::from(response.clone())));
        let body = std::fs::read_to_string(get_tasks_print_path).unwrap();
        let mut response = http::response::Response::new(body.clone());
        *response.status_mut() = get_tasks_print_status;
        requester
            .expect_get_tasks_print()
            .times(get_tasks_print_call)
            .returning(move |_| Ok(Response::from(response.clone())));

        let atcoder = Atcoder::new(requester);
        let result = atcoder.get_problems_detail(args_contest_id.to_string());

        if expected.is_err() {
            assert_eq!(result, expected);
            return Ok(());
        }
        let expected_problems = expected.unwrap();
        expected_problems.iter().enumerate().for_each(|(i, e)| {
            let problem = &result.as_ref().unwrap()[i];
            assert_eq!(problem.title, e.title);
            assert_eq!(problem.id, e.id);
            assert_eq!(problem.code, e.code);
            assert_eq!(problem.statement, e.statement);
            assert_eq!(problem.constraints, e.constraints);
            assert_eq!(problem.input_format, e.input_format);
            assert_eq!(problem.samples, e.samples);
            assert_eq!(problem.point, e.point);
            assert_eq!(problem.time_limit, e.time_limit);
            assert_eq!(problem.memory_limit, e.memory_limit);
        });
        assert_eq!(result.unwrap(), expected_problems);
        Ok(())
    }
}
