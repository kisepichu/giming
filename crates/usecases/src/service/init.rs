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

        if self.repository.contest_repo().exists(&contest_id)?
            && !self
                .repository
                .contest_repo()
                .exists_unstarted(&contest_id)?
        {
            return Ok(InitResult { created: false });
        }

        loop {
            match self.online_judge.get_problems_detail(&contest_id) {
                Ok(problems) => {
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
                        .create(&contest_id, &workspace)?;

                    return Ok(InitResult { created: true });
                }
                Err(e) => {
                    println!("get problems failed: {}", e);
                }
            }

            if !self.repository.contest_repo().exists(&contest_id)? {
                self.repository
                    .contest_repo()
                    .create_unstarted(&contest_id)?;
                return Ok(InitResult { created: false });
            }
            self.online_judge.wait_for_start(&contest_id)?;
        }
    }
}

mod test {}
