pub mod online_judge;

use std::marker::PhantomData;

use online_judge::{LoginArgs, OnlineJudge};

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<'o, E, O: OnlineJudge<E>> {
    online_judge: &'o O,
    _phantom: PhantomData<E>,
}

impl<'o, E, O: OnlineJudge<E>> Service<'o, E, O> {
    pub fn new(online_judge: &'o O) -> Self {
        Self {
            online_judge,
            _phantom: PhantomData,
        }
    }
    pub fn login(&self, args: LoginArgs) -> Result<(), E> {
        self.online_judge.login(args)
    }
    pub fn init(&self, _args: InitArgs) -> Result<(), E> {
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        todo!()
    }
}
