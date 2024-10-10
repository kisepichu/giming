use std::io::{BufRead, Write};
use std::iter::once;

use domain::error::Error;
use interfaces::controller::Controller;
use usecases::service::online_judge::OnlineJudge;

use clap::Parser;

use crate::error::DetailError;

pub mod commands;
use commands::{Cli, Command, ShellCommand};
mod login;

fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("http") {
        contest_id_or_url.split('/').last().unwrap().to_string()
    } else {
        contest_id_or_url
    }
}

pub struct Shell<O: OnlineJudge<DetailError>> {
    controller: Controller<DetailError, O>,
    prompt: String,
    contest_id: String,
}

impl<O: OnlineJudge<DetailError>> Shell<O> {
    pub fn new(online_judge: O, prompt: String, cli: &Cli) -> Self {
        Self {
            controller: Controller::new(online_judge),
            prompt,
            contest_id: to_contest_id(cli.contest.clone()),
        }
    }
    fn print_prompt(&self) {
        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &self.contest_id);
        let mut tera = tera::Tera::default();
        print!(
            "{}",
            tera.render_str(&self.prompt, &prompt_context).unwrap()
        );
        std::io::stdout().flush().unwrap();
    }
    pub fn run(&self) -> i32 {
        let mut stdin_iter = std::io::stdin().lock().lines();

        self.print_prompt();
        while let Some(r) = stdin_iter.next() {
            match ShellCommand::try_parse_from(once("").chain(r.unwrap().split_whitespace())) {
                Ok(shell) => match shell.command {
                    Command::Exit(exit_args) => {
                        if exit_args.code == 0 {
                            println!("bye");
                        }
                        return exit_args.code;
                    }
                    Command::Login(login_args) => {
                        self.login(&mut stdin_iter, login_args).unwrap_or_else(|e| {
                            eprintln!("{}", e.error_chain());
                        });
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
