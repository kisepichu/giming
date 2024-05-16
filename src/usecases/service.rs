use crate::domain::error::Result;

pub mod online_judge;

use online_judge::{LoginArgs, OnlineJudge};

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<'o, O> {
    online_judge: &'o O,
}

impl<'o, O: OnlineJudge> Service<'o, O> {
    pub fn new(online_judge: &'o O) -> Self {
        Self { online_judge }
    }
    pub fn login(&self, args: LoginArgs) -> Result<()> {
        self.online_judge.login(args)
    }
    pub fn init(&self, _args: InitArgs) -> Result<()> {
        // self.online_judge.get_contest 等使いコンテストディレクトリを作るロジックを書く
        todo!()
    }
}
