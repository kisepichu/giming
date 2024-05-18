pub mod error;
pub mod online_judge;

use domain::error::Error;

use std::marker::PhantomData;

use online_judge::{LoginArgs, OnlineJudge};

use self::error::ServiceError;

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<E: Error + 'static, O: OnlineJudge<E>> {
    online_judge: O,
    _phantom: PhantomData<E>,
}

impl<E: Error + 'static, O: OnlineJudge<E>> Service<E, O> {
    pub fn new(online_judge: O) -> Self {
        Self {
            online_judge,
            _phantom: PhantomData,
        }
    }
    pub fn login(&self, args: LoginArgs) -> Result<(), Box<ServiceError<E>>> {
        self.online_judge.login(args)
    }
    pub fn init(&self, _args: InitArgs) -> Result<(), Box<ServiceError<E>>> {
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        todo!()
    }
}
