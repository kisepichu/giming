use std::io::{BufRead, Write};

use clap::{Parser, Subcommand};

use crate::infrastructure::repository_impl::atcoder::AtcoderRepository;
use crate::usecases::service::atcoder::{Atcoder, LoginArgs};

pub struct ShellConfig {
    pub prompt: String,
}

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
#[command(name = "")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub struct ExitShellArgs {
    #[clap(default_value = "0")]
    pub code: i32,
}

#[derive(Parser, Debug)]
pub struct LoginShellArgs {
    #[clap(default_value = "")]
    pub username: String,
    #[clap(default_value = "")]
    pub password: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Exit
    Exit(ExitShellArgs),
    /// Login
    Login(LoginShellArgs),
}

// コンテストの url または id から、 id に変換する TODO
fn to_contest_id(_: String) -> String {
    "abc998".to_string()
}

// ファイルからシェルの設定を読み込む TODO
pub fn get_settings() -> ShellConfig {
    ShellConfig {
        prompt: "{{contest_id}}> ".to_string(),
    }
}

pub fn run(contest: String) -> i32 {
    let contest_id = to_contest_id(contest);

    let config = get_settings();

    let mut tera_context = tera::Context::new();
    tera_context.insert("contest_id", &contest_id);
    let mut tera = tera::Tera::default();
    let prompt = tera.render_str(&config.prompt, &tera_context).unwrap();

    let atcoder_repository = AtcoderRepository::new();
    let atcoder = Atcoder::new(&atcoder_repository);

    let mut stdin_iter = std::io::stdin().lock().lines();
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        match stdin_iter.next() {
            Some(r) => {
                match Cli::try_parse_from(std::iter::once("").chain(r.unwrap().split_whitespace()))
                {
                    Ok(cli) => match cli.command {
                        Command::Exit(exit_args) => {
                            if exit_args.code == 0 {
                                println!("bye");
                            }
                            return exit_args.code;
                        }
                        Command::Login(login_args) => {
                            match atcoder.login(LoginArgs {
                                username: login_args.username,
                                password: login_args.password,
                            }) {
                                Ok(_) => println!("login success"),
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                    },
                    Err(e) => println!("{}", e),
                }
            }
            None => {
                break;
            }
        }
    }
    0
}
