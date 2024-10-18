use domain::error::Error;

use crate::{online_judge::OnlineJudge, service_error::ServiceError};

use super::Service;

impl<E: Error + 'static> Service<E> {
    pub fn init(
        &mut self,
        contest_id: String,
        oj_switch: Option<Box<dyn OnlineJudge<E>>>,
    ) -> Result<(), ServiceError<E>> {
        if let Some(oj) = oj_switch {
            self.online_judge = oj;
        }
        let problems = self.online_judge.get_problems_detail(&contest_id)?;
        // self.directory_generator.generate(&contest_id, problems)?;
        self.repository
            .contest_repo()
            .create_if_not_exists(&contest_id, problems)?;
        todo!()
    }
}

mod test {}
