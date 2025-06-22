use clap::Subcommand;
use crate::{init, processer, config};

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// nix run shell expansion
    Run(processer::Run),
    /// nix shell shell expansion
    Shell(processer::Shell),
    /// nix develop shell expansion
    Develop(processer::Develop),
    /// Copies flake templates from ~/.config/nf/templates/<name> to ./flake.nix
    Init(init::Init),
    /// Manages the config, usually found in ~/.config/nf
    Config(config::Config),
}
