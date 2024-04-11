use clap::{Parser, Subcommand};

mod init;
mod terminal;

/// ac-rs: Contest directory generator
/// init: Create contest directory
/// console: Start contest console

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create contest directory
    Init(init::InitArgs),
    #[command(alias = "t")]
    /// Start contest terminal
    Terminal(terminal::TerminalArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            init::init(args);
        }
        Commands::Terminal(args) => {
            terminal::terminal(args);
        }
    }
}
