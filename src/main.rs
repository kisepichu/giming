use clap::Parser;

use ac_rs::infrastructure::shell;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(default_value = "")]
    contest: String,
}

fn main() {
    let cli = Cli::parse();

    let code = shell::run(cli.contest);

    std::process::exit(code);
}
