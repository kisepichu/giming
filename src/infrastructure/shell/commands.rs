use clap::{Parser, Subcommand};

use crate::interfaces::controller::input::{ExitInput, LoginInput};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(default_value = "")]
    pub contest: String,
    #[arg(default_value = "{{ contest_id }}>")]
    pub prompt: String,
}

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
#[command(name = "")]
pub struct ShellCommand {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub struct ExitCommand {
    #[clap(default_value = "0")]
    pub code: i32,
}

impl ExitInput for ExitCommand {
    fn code(&self) -> i32 {
        self.code
    }
}

#[derive(Parser, Debug)]
pub struct LoginCommand {
    #[clap(default_value = "")]
    pub username: String,
    #[clap(default_value = "")]
    pub password: String,
}

impl LoginInput for LoginCommand {
    fn username(&self) -> String {
        self.username.clone()
    }
    fn password(&self) -> String {
        self.password.clone()
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Exit
    Exit(ExitCommand),
    /// Login
    ///
    /// if username is not provided as arguments, they will be read from environment variables ATCODER_USERNAME and ATCODER_PASSWORD
    Login(LoginCommand),
}
