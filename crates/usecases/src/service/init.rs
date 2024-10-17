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
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        let problems = self.online_judge.get_problems_summary(contest_id)?;
        println!("problems: \n{:?}", problems);
        todo!()
    }
}

mod test {}
