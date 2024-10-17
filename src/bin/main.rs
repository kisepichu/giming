use clap::Parser;
use domain::error::ResultChain;
use infrastructure::{
    config::Config,
    shell::{commands::Cli, Shell},
};

fn main() {
    let cfg = Config::load().unwrap_chain();

    let cli = Cli::parse();

    let mut shell = Shell::new(&cli, cfg).unwrap_chain();

    std::process::exit(shell.run());
}
