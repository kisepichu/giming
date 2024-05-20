use clap::Parser;
use domain::error::ResultChain;
use infrastructure::{
    external::atcoder_requester::atcoder_requester_impl::AtcoderRequesterImpl,
    online_judge_impl::atcoder::Atcoder,
    shell::{commands::Cli, Shell},
};

fn main() {
    let cli = Cli::parse();

    let atcoder_requester = AtcoderRequesterImpl::new().unwrap_chain();
    let atcoder = Atcoder::new(atcoder_requester);
    let shell = Shell::new(atcoder, "{{contest_id}}> ".to_string(), &cli);

    std::process::exit(shell.run());
}
