use crate::domain::error::Result;

use super::repository::LoginArgs;

pub struct InitArgs {
    pub contest_id: String,
}

pub trait Service {
    fn login(&self, args: LoginArgs) -> Result<()>;
    fn init(&self, args: InitArgs) -> Result<()>;
}
