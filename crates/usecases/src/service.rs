pub mod init;
pub mod login;
pub mod whoami;

use domain::error::Error;

use std::marker::PhantomData;

use crate::{online_judge::OnlineJudge, repository::Repository};

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<E: Error + 'static> {
    online_judge: Box<dyn OnlineJudge<E>>,
    repository: Box<dyn Repository<E>>,
    _phantom: PhantomData<E>,
}

impl<E: Error + 'static> Service<E> {
    pub fn new(oj: Box<dyn OnlineJudge<E>>, repository: Box<dyn Repository<E>>) -> Self {
        Self {
            online_judge: oj,
            repository,
            _phantom: PhantomData,
        }
    }
    pub fn online_judge_name(&self) -> &str {
        self.online_judge.name()
    }
}
