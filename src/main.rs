use clap::{Parser, Subcommand};

/// ac-rs: Contest directory generator
/// init: Create contest directory
/// sh: Start contest console

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
    Init(InitArgs),
    /// Start contest console
    Sh(ShArgs),
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct InitArgs {
    /// Contest id or url
    #[clap(short, long)]
    contest: String,

    /// Solution files to overwrite when exists
    #[clap(short, long, default_value = "")]
    overwrite: String,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct ShArgs {
    /// Contest id or url
    #[clap(short, long)]
    contest: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            println!("Init: {:?}", args);
        }
        Commands::Sh(args) => {
            println!("Sh: {:?}", args);
        }
    }
}
