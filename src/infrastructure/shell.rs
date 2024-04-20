use std::io::{BufRead, Write};

use clap::Parser;

use crate::infrastructure::model::shell;

// コンテストの url または id から、 id に変換する TODO
fn to_contest_id(_: String) -> String {
    "abc998".to_string()
}

// ファイルからシェルの設定を読み込む TODO
pub fn get_settings() -> shell::ShellConfig {
    shell::ShellConfig {
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

    let mut stdin_iter = std::io::stdin().lock().lines();
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        match stdin_iter.next() {
            Some(r) => {
                match shell::Cli::try_parse_from(
                    std::iter::once("").chain(r.unwrap().split_whitespace()),
                ) {
                    Ok(cli) => match cli.command {
                        shell::Command::Exit(exit_args) => {
                            if exit_args.code == 0 {
                                println!("bye");
                            }
                            return exit_args.code;
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
