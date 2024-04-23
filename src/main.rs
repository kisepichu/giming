use clap::Parser;

use ac_rs::interfaces::shell::{self, Cli};

fn main() {
    let code = shell::run(Cli::parse());

    std::process::exit(code);
}
