use crate::{
    domain::error::Result,
    usecases::{repository::LoginArgs, service::Service},
};

use self::dto::LoginArgsDTO;
pub mod dto;

pub struct Controller<'s, S: Service> {
    pub service: &'s S,
}

impl<'s, S: Service> Controller<'s, S> {
    pub fn new(service: &'s S) -> Self {
        Self { service }
    }
    pub fn login(&self, args: LoginArgsDTO) -> Result<()> {
        self.service.login(LoginArgs {
            username: args.username,
            password: args.password,
        })
    }
}
