use clap::Subcommand;
use crate::{init, processer, config};

#[derive(Debug, Subcommand)]
pub enum Mode {
    Run(processer::Run),
    Shell(processer::Shell),
    Develop(processer::Develop),
    Init(init::Init),
    Config(config::Config),
}
