use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct TerminalArgs {}

pub fn terminal(args: TerminalArgs) {
    println!("Terminal: {:?}", args);
}
