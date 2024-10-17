mod init;
mod login;
mod whoami;

use domain::error::Error;

use std::marker::PhantomData;

use crate::online_judge::OnlineJudge;

pub struct InitArgs {
    pub contest_id: String,
}

pub struct Service<E: Error + 'static> {
    online_judge: Box<dyn OnlineJudge<E>>,
    _phantom: PhantomData<E>,
}

impl<E: Error + 'static> Service<E> {
    pub fn new(oj: Box<dyn OnlineJudge<E>>) -> Self {
        Self {
            online_judge: oj,
            _phantom: PhantomData,
        }
    }
}
