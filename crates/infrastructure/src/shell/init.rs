use domain::error::Error;
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;

use super::{commands::InitCommand, oj_from_contest_id, to_contest_id, Shell};

impl Shell {
    pub fn init(&mut self, args: InitCommand) {
        let contest_id = to_contest_id(args.contest_id);
        let oj_switch = match oj_from_contest_id(&contest_id, self.controller.online_judge_name()) {
            Ok(o) => Some(o),
            Err(e) => {
                if e != "same online judge" {
                    eprintln!(
                        "{}",
                        ServiceError::InstantiateFailed(DetailError::Custom(e)).error_chain()
                    );
                    return;
                }
                None
            }
        };

        match self.controller.init(
            InitCommand {
                contest_id: contest_id.clone(),
            },
            oj_switch,
        ) {
            Ok(r) => {
                if r.created {
                    println!("workspace {} is initialized.", contest_id);
                } else {
                    println!("Opened workspace {}.\n  - tip: Run `new-solution <PROBLEM>` to recreate solution, or `new-workspace` to recreate workspace. Old ones will be archived.", contest_id);
                }
            }
            Err(e) => {
                eprintln!("{}", e.error_chain());
            }
        }
    }
}
