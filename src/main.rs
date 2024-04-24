use ac_rs::{
    infrastructure::{
        repository_impl::RepositoryImpl,
        shell::{Cli, Shell},
    },
    usecases::service_impl::ServiceImpl,
};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let repository = RepositoryImpl::new();
    let service = ServiceImpl::new(&repository);
    let shell = Shell::new(&service, "{{contest_id}}> ".to_string(), &cli);

    std::process::exit(shell.run());
}
