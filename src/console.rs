use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ConsoleArgs {
    /// Contest id or url
    #[clap(short, long)]
    contest: String,
}

pub fn console(args: ConsoleArgs) {
    println!("Console: {:?}", args);
}
