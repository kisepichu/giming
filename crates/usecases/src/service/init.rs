use domain::error::Error;

use crate::service_error::ServiceError;

use super::Service;

impl<E: Error + 'static> Service<E> {
    pub fn init(&self, contest_id: String) -> Result<(), ServiceError<E>> {
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        let _problems = self.online_judge.get_problems_detail(contest_id)?;
        todo!()
    }
}
