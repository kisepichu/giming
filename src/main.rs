use clap::Parser;

use ac_rs::infrastructure::shell::{self, Cli};

fn main() {
    let code = shell::run(Cli::parse());

    std::process::exit(code);
}
