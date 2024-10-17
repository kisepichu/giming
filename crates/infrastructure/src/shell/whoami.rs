use domain::error::Error;

use super::{commands::WhoamiCommand, Shell};

impl Shell {
    pub fn whoami(&self, args: WhoamiCommand) {
        let username = match self.controller.whoami(args) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e.error_chain());
                println!("not logged in");
                return;
            }
        };
        println!(
            "username for {} is {}",
            self.controller.online_judge_name(),
            username
        );
    }
}
