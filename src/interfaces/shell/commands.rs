use clap::{Parser, Subcommand};

use crate::{
    infrastructure::repository_impl::RepositoryImpl,
    usecases::service::{LoginArgs, Service},
};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(default_value = "")]
    pub contest: String,
}

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
#[command(name = "")]
pub struct Shell {
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

pub fn login(args: LoginShellArgs, atcoder: &Service<'_, RepositoryImpl>) {
    match atcoder.login(LoginArgs {
        username: args.username,
        password: args.password,
    }) {
        Ok(_) => println!("login success"),
        Err(e) => eprintln!("{}", e),
    }
}
