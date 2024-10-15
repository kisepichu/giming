use usecases::error::ServiceError;

use crate::error::DetailError;

use super::{commands::InitCommand, Shell};

impl Shell {
    pub fn init(&self, args: InitCommand) -> Result<(), ServiceError<DetailError>> {
        self.controller.init(InitCommand {
            contest_id: args.contest_id,
        })
    }
}
