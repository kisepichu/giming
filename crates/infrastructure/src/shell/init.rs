use domain::error::Error;
use usecases::service_error::ServiceError;

use crate::detail_error::DetailError;
use tera::Tera;

use super::{Shell, commands::InitCommand, oj_from_contest_id, to_contest_id};

impl Shell {
    fn open(&self, contest_id: &str) {
        let mut tera = Tera::default();
        let mut tera_context = tera::Context::new();
        tera_context.insert("contest_dir", &self.config.solutions_root);
        tera_context.insert("contest_id", contest_id);
        match tera.render_str(&self.config.open_command, &tera_context) {
            Ok(command) => println!(
                "{}",
                match system::system(command.as_str()) {
                    Ok(_) => "executing open_command",
                    Err(_) => "error executing open_command",
                }
            ),
            Err(e) => {
                eprintln!("error rendering open_command: {}", e);
            }
        }
    }

    pub fn init(&mut self, args: InitCommand) {
        let contest_id = to_contest_id(args.contest_id.clone());
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
                    println!(
                        "Opened workspace {}.\n  - tip: Run `new-solution <PROBLEM>` to recreate solution, or `new-workspace` to recreate workspace. Old ones will be archived.",
                        contest_id
                    );
                }
                self.contest_id = args.contest_id;

                if std::env::var("TERM_PROGRAM").unwrap_or_default() != "vscode" {
                    self.open(&contest_id);
                    std::process::exit(0);
                }
            }
            Err(e) => {
                eprintln!("{}", e.error_chain());
            }
        }
    }
}
