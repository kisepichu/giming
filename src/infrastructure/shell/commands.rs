use clap::{Parser, Subcommand};

use crate::{
    infrastructure::repository_impl::RepositoryImpl,
    interfaces::controller::dto::{ExitArgsDTO, LoginArgsDTO},
    usecases::{repository::LoginArgs, service::Service, service_impl::ServiceImpl},
};

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

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Exit
    Exit(ExitArgsDTO),
    /// Login
    Login(LoginArgsDTO),
}
