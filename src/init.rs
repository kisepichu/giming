use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct InitArgs {
    /// Contest id or url
    #[clap(short, long)]
    contest: String,

    /// Solution files to overwrite when exists
    #[clap(short, long, default_value = "")]
    overwrite: String,
}

pub fn init(args: InitArgs) {
    println!("Init: {:?}", args);
}
