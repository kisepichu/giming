use clap::Parser;
use confy::ConfyError;
use domain::error::ResultChain;
use infrastructure::{
    config::Config,
    external::atcoder_requester::atcoder_requester_impl::AtcoderRequesterImpl,
    online_judge_impl::atcoder::Atcoder,
    shell::{commands::Cli, Shell},
};

fn main() -> Result<(), ConfyError> {
    let cfg = Config::load().unwrap_chain();

    let cli = Cli::parse();

    let atcoder_requester = AtcoderRequesterImpl::new().unwrap_chain();
    let atcoder = Atcoder::new(atcoder_requester);
    let shell = Shell::new(atcoder, cfg.prompt, &cli);

    std::process::exit(shell.run());
}
