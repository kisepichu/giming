use clap::{Parser, Subcommand};

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
pub struct ExitArgs {
    #[clap(default_value = "0")]
    pub code: i32,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Exit
    Exit(ExitArgs),
}
