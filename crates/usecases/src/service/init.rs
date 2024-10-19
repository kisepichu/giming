use domain::{
    entity::{WorkProblem, Workspace},
    error::Error,
};

use crate::{io_inferrer::IOInferrer, online_judge::OnlineJudge, service_error::ServiceError};

use super::Service;

pub struct InitResult {
    pub created: bool,
}

impl<E: Error + 'static> Service<E> {
    pub fn init(
        &mut self,
        contest_id: String,
        oj_switch: Option<Box<dyn OnlineJudge<E>>>,
    ) -> Result<InitResult, ServiceError<E>> {
        if let Some(oj) = oj_switch {
            self.online_judge = oj;
        }

        if self.repository.contest_repo().exists(&contest_id)? {
            return Ok(InitResult { created: false });
        }

        let problems = self.online_judge.get_problems_detail(&contest_id)?;
        let work_problems = problems
            .iter()
            .map(|p| WorkProblem {
                problem: p,
                io_spec: IOInferrer::infer(p),
            })
            .collect();
        let workspace = Workspace {
            contest_id: contest_id.clone(),
            work_problems,
        };
        self.repository
            .contest_repo()
            .create(&contest_id, workspace)?;
        Ok(InitResult { created: true })
    }
}

mod test {}
