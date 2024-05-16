use crate::{
    domain::error::Result,
    usecases::service::{
        online_judge::{LoginArgs, OnlineJudge},
        Service,
    },
};

pub mod input;
use input::LoginInput;

pub struct Controller<'o, O: OnlineJudge> {
    pub service: Service<'o, O>,
}

impl<'o, O: OnlineJudge> Controller<'o, O> {
    pub fn new(online_judge: &'o O) -> Self {
        Self {
            service: Service::new(online_judge),
        }
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<()> {
        self.service.login(LoginArgs {
            username: args.username(),
            password: args.password(),
        })
    }
}
