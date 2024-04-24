use crate::{
    domain::error::Result,
    usecases::{repository::LoginArgs, service::Service},
};

use self::input::LoginInput;
pub mod input;

pub struct Controller<'s, S: Service> {
    pub service: &'s S,
}

impl<'s, S: Service> Controller<'s, S> {
    pub fn new(service: &'s S) -> Self {
        Self { service }
    }
    pub fn login<T: LoginInput>(&self, args: T) -> Result<()> {
        self.service.login(LoginArgs {
            username: args.username(),
            password: args.password(),
        })
    }
}
