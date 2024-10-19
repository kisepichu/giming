use clap::Parser;
use domain::error::ResultChain;
use infrastructure::{
    config_impl::ConfigImpl,
    shell::{commands::Cli, Shell},
};

fn main() {
    let config = Box::leak(Box::new(ConfigImpl::load().unwrap_chain()));

    let cli = Cli::parse();

    let mut shell = Shell::new(&cli, config).unwrap_chain();

    std::process::exit(shell.run().unwrap());
}
