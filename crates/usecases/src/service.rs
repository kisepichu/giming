mod init;
mod login;
mod whoami;

use domain::error::Error;

use std::marker::PhantomData;

use crate::{directory_generator::DirectoryGenerator, online_judge::OnlineJudge};

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<E: Error + 'static> {
    online_judge: Box<dyn OnlineJudge<E>>,
    directory_generator: Box<dyn DirectoryGenerator<E>>,
    contest_id: String,
    _phantom: PhantomData<E>,
}

impl<E: Error + 'static> Service<E> {
    pub fn new(
        oj: Box<dyn OnlineJudge<E>>,
        directory_generator: Box<dyn DirectoryGenerator<E>>,
        contest_id: String,
    ) -> Self {
        Self {
            online_judge: oj,
            directory_generator,
            contest_id,
            _phantom: PhantomData,
        }
    }
    pub fn online_judge_name(&self) -> &str {
        self.online_judge.name()
    }
    pub fn contest_id(&self) -> String {
        self.contest_id.clone()
    }
}
