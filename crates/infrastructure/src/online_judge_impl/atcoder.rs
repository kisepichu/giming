use crate::detail_error::DetailError;
use crate::external::atcoder_requester::AtcoderRequester;

use domain::entity::{Problem, ProblemSummary};
use usecases::{online_judge::OnlineJudge, service_error::ServiceError};

pub mod get_problems_detail;
pub mod get_problems_summary;
pub mod login;
pub mod submit;
pub mod wait_for_start;
pub mod whoami;

pub struct Atcoder<R: AtcoderRequester> {
    requester: R,
}

impl<R: AtcoderRequester> Atcoder<R> {
    pub fn new(requester: R) -> Self {
        Self { requester }
    }
}

impl<R: AtcoderRequester> OnlineJudge<DetailError> for Atcoder<R> {
    fn name(&self) -> &str {
        "AtCoder"
    }
    fn whoami(&self) -> Result<String, ServiceError<DetailError>> {
        self.whoami()
    }
    fn login(&self, username: String, password: String) -> Result<(), ServiceError<DetailError>> {
        self.login(username, password)
    }
    fn wait_for_start(&self, contest_id: &str) -> Result<(), ServiceError<DetailError>> {
        self.wait_for_start(contest_id)
    }
    fn get_problems_summary(
        &self,
        contest_id: &str,
    ) -> Result<Vec<ProblemSummary>, ServiceError<DetailError>> {
        self.get_problems_summary(contest_id)
    }
    fn get_problems_detail(
        &self,
        contest_id: &str,
    ) -> Result<Vec<Problem>, ServiceError<DetailError>> {
        self.get_problems_detail(contest_id)
    }
    fn submit(&self, solution_id: String) -> Result<(), ServiceError<DetailError>> {
        self.submit(solution_id)
    }
}
