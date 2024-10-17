use std::io::{BufRead, Write};
use std::iter::once;

use domain::error::Error;
use interfaces::controller::Controller;

use clap::Parser;
use usecases::online_judge::OnlineJudge;
use usecases::service_error::ServiceError;

use crate::config::Config;
use crate::detail_error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::AtcoderRequesterImpl;
use crate::online_judge_impl::atcoder::Atcoder;

pub mod commands;
use commands::{Cli, Command, ShellCommand};
mod init;
mod login;
mod whoami;

fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("http") {
        contest_id_or_url.split('/').last().unwrap().to_string()
    } else {
        contest_id_or_url
    }
}

// current と同じだった場合や、判定できなかった場合に None を返す todo 分ける
fn oj_from_contest_id(
    _contest_id: &str,
    current: &str,
) -> Option<Box<dyn OnlineJudge<DetailError>>> {
    // todo
    if current == "AtCoder" {
        return None;
    }
    let atcoder_requester = match AtcoderRequesterImpl::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e.error_chain());
            return None;
        }
    };
    let atcoder = Atcoder::new(atcoder_requester);
    Some(Box::new(atcoder))
}

pub struct Shell {
    controller: Controller<DetailError>,
    prompt: String,
}

impl Shell {
    pub fn new(cli: &Cli, cfg: Config) -> Result<Self, ServiceError<DetailError>> {
        let contest_id = to_contest_id(cli.contest.clone());
        let oj = oj_from_contest_id(&contest_id, "").ok_or(ServiceError::InstantiateFailed(
            DetailError::Custom(format!(
                "cannot determine the type of online judge for {}",
                contest_id,
            )),
        ))?;
        Ok(Self {
            controller: Controller::new(oj, contest_id),
            prompt: cfg.prompt,
        })
    }
    fn print_prompt(&self) {
        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &self.controller.contest_id());
        let mut tera = tera::Tera::default();
        print!(
            "{}",
            tera.render_str(&self.prompt, &prompt_context).unwrap()
        );
        std::io::stdout().flush().unwrap();
    }
    pub fn run(&mut self) -> i32 {
        let mut stdin_iter = std::io::stdin().lock().lines();

        self.print_prompt();
        while let Some(r) = stdin_iter.next() {
            match ShellCommand::try_parse_from(once("").chain(r.unwrap().split_whitespace())) {
                Ok(shell) => match shell.command {
                    Command::Exit(args) => {
                        if args.code == 0 {
                            println!("bye");
                        }
                        return args.code;
                    }
                    Command::Whoami(args) => {
                        self.whoami(args);
                    }
                    Command::Init(args) => {
                        self.init(args);
                    }
                    Command::Login(args) => {
                        self.login(&mut stdin_iter, args);
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            self.print_prompt();
        }
        0
    }
}
