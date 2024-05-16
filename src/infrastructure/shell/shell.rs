use std::io::{BufRead, Write};
use std::iter::once;
use std::{env, io};

use crate::interfaces::controller::Controller;
use crate::usecases::service::online_judge::OnlineJudge;

use clap::Parser;
use rpassword::read_password;

use super::commands::{Cli, Command, LoginCommand, ShellCommand};

fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("http") {
        contest_id_or_url.split("/").last().unwrap().to_string()
    } else {
        contest_id_or_url
    }
}

pub struct Shell<'o, O: OnlineJudge> {
    controller: Controller<'o, O>,
    prompt: String,
}

impl<'o, O: OnlineJudge> Shell<'o, O> {
    pub fn new(online_judge: &'o O, prompt: String, cli: &Cli) -> Self {
        let contest_id = to_contest_id(cli.contest.clone());

        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &contest_id);
        let mut tera = tera::Tera::default();
        Self {
            controller: Controller::new(online_judge),
            prompt: tera.render_str(&prompt, &prompt_context).unwrap(),
        }
    }
    fn print_prompt(&self) {
        print!("{}", self.prompt);
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
                        self.login(login_args);
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

    fn login(&self, login_args: LoginCommand) {
        let username = login_args.username;
        let username = if username.is_empty() {
            match env::var("ATCODER_USERNAME") {
                Ok(username) => username,
                Err(_) => {
                    eprintln!("ATCODER_USERNAME not set\n\nFor more information, run 'help login'");
                    return;
                }
            }
        } else {
            username
        };
        let password = login_args.password;
        let password = if password.is_empty() {
            // input from stdin
            let mut password: String = String::new();
            while password.is_empty() {
                print!("password for {}: ", username);
                io::stdout().flush().unwrap();
                password = read_password().unwrap_or("".to_string());
            }
            password
        } else {
            password
        };
        self.controller
            .login(LoginCommand { username, password })
            .unwrap_or_else(|e| eprintln!("{}", e));
    }
}
