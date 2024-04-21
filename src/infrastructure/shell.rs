use std::io::{BufRead, Write};

use crate::infrastructure::repository_impl::atcoder::AtcoderRepository;
use crate::usecases::service::atcoder::Atcoder;

use clap::Parser;
mod commands;
pub use commands::Cli;
use commands::*;

pub struct ShellConfig {
    pub prompt: String,
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

pub fn run(cli: Cli) -> i32 {
    let contest_id = to_contest_id(cli.contest);

    let config = get_settings();

    let prompt = {
        let mut prompt_context = tera::Context::new();
        prompt_context.insert("contest_id", &contest_id);
        let mut tera = tera::Tera::default();
        tera.render_str(&config.prompt, &prompt_context).unwrap()
    };

    let atcoder_repository = AtcoderRepository::new();
    let atcoder = Atcoder::new(&atcoder_repository);

    let mut stdin_iter = std::io::stdin().lock().lines();
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        match stdin_iter.next() {
            Some(r) => {
                match Shell::try_parse_from(
                    std::iter::once("").chain(r.unwrap().split_whitespace()),
                ) {
                    Ok(shell) => match shell.command {
                        Command::Exit(exit_args) => {
                            if exit_args.code == 0 {
                                println!("bye");
                            }
                            return exit_args.code;
                        }
                        Command::Login(login_args) => {
                            login(login_args, &atcoder);
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
