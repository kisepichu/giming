use clap::{Parser, Subcommand};

use interfaces::controller::input::{ExitInput, InitInput, LoginInput};

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
    #[clap(default_value = "atcoder")]
    pub online_judge: String,
}

impl LoginInput for LoginCommand {
    fn username(&self) -> String {
        self.username.clone()
    }
    fn password(&self) -> String {
        self.password.clone()
    }
}

#[derive(Parser, Debug)]
pub struct InitCommand {
    pub contest_id: String,
}

impl InitInput for InitCommand {
    fn contest_id(&self) -> String {
        self.contest_id.clone()
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Exit
    #[clap(visible_aliases = ["quit", "q"])]
    Exit(ExitCommand),
    /// Login to the online judge
    ///
    /// Please set the following envvars to avoid prompting:
    /// `ATCODER_USERNAME` and `ATCODER_PASSWORD` for AtCoder
    #[clap(visible_aliases = ["l"])]
    Login(LoginCommand),
    /// Initialize the directory structure for the contest
    #[clap(visible_aliases = ["i"])]
    Init(InitCommand),
}
