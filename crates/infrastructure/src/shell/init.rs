use domain::error::Error;

use super::{commands::InitCommand, Shell};

impl Shell {
    pub fn init(&self, args: InitCommand) {
        match self.controller.init(InitCommand {
            contest_id: args.contest_id,
        }) {
            Ok(_) => println!("init ok"),
            Err(e) => {
                eprintln!("{}", e.error_chain());
            }
        }
    }
}
