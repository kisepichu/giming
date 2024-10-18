use std::io::{BufRead, Write};
use std::iter::once;

use domain::error::Error;
use interfaces::controller::Controller;

use clap::Parser;
use usecases::online_judge::OnlineJudge;
use usecases::service_error::ServiceError;

use crate::config_impl::ConfigImpl;
use crate::detail_error::DetailError;
use crate::external::atcoder_requester::atcoder_requester_impl::AtcoderRequesterImpl;
use crate::online_judge_impl::atcoder::Atcoder;
use crate::repository_impl::RepositoryImpl;

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

fn oj_from_contest_id(
    _contest_id: &str,
    current: &str,
) -> Result<Box<dyn OnlineJudge<DetailError>>, String> {
    // todo
    if current == "AtCoder" {
        return Err("same online judge".to_string());
    }
    let atcoder_requester = match AtcoderRequesterImpl::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e.error_chain());
            return Err("oj_from_contest_id failed: AtcoderRequesterImpl::new()".to_string());
        }
    };
    let atcoder = Atcoder::new(atcoder_requester);
    Ok(Box::new(atcoder))
    // Err(format!(
    //     "cannot determine the type of online judge for {}",
    //     contest_id,
    // ))
}

pub struct Shell {
    controller: Controller<DetailError>,
    config: &'static ConfigImpl,
}

impl Shell {
    pub fn new(cli: &Cli, config: &'static ConfigImpl) -> Result<Self, ServiceError<DetailError>> {
        let contest_id = to_contest_id(cli.contest.clone());
        let oj = match oj_from_contest_id(&contest_id, "") {
            Ok(o) => o,
            Err(e) => {
                return Err(ServiceError::InstantiateFailed(DetailError::Custom(e)));
            }
        };
        let repository = RepositoryImpl::new(config);

        Ok(Self {
            controller: Controller::new(oj, Box::new(repository), contest_id),
            config,
        })
    }
    fn print_prompt(&self) {
        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &self.controller.contest_id());
        let mut tera = tera::Tera::default();
        print!(
            "{}",
            tera.render_str(&self.config.prompt, &prompt_context)
                .unwrap()
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
