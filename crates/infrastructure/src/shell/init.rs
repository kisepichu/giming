use domain::error::Error;

use super::{commands::InitCommand, oj_from_contest_id, to_contest_id, Shell};

impl Shell {
    pub fn init(&mut self, args: InitCommand) {
        let contest_id = to_contest_id(args.contest_id);
        let oj_switch = oj_from_contest_id(&contest_id, self.controller.online_judge_name());

        match self.controller.init(InitCommand { contest_id }, oj_switch) {
            Ok(_) => println!("initj ok"),
            Err(e) => {
                eprintln!("{}", e.error_chain());
            }
        }
    }
}
