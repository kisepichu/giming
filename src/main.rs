use clap::{Parser, Subcommand};

mod console;
mod init;

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
    /// Start contest console
    Console(console::ConsoleArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            init::init(args);
        }
        Commands::Console(args) => {
            console::console(args);
        }
    }
}
