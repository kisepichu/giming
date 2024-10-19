use domain::error::Error;
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

use super::{commands::WhoamiCommand, Shell};

impl Shell {
    pub fn whoami(&self, args: WhoamiCommand) {
        let username = match self.controller.whoami(args) {
            Ok(s) => s,
            Err(e) => {
                if let ServiceError::WhoamiFailed(DetailError::ParsingElementNotFound(_)) = e {
                    println!("not logged in");
                    return;
                }
                eprintln!("{}", e.error_chain());
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
