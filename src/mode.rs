use clap::Subcommand;
use crate::processer;

#[derive(Debug, Subcommand)]
pub enum Mode {
    Run(processer::Run),
    Shell(processer::Shell),
    Develop(processer::Develop),
}
