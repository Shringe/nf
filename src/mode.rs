use clap::Subcommand;
use crate::{config, init, processer, cli::Actionable};

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

impl Actionable for Mode {
    fn perform(&self, debug: bool) {
        match self {
            Mode::Run(run) => run.perform(debug),
            Mode::Shell(shell) => shell.perform(debug),
            Mode::Develop(develop) => develop.perform(debug),
            Mode::Init(init) => init.perform(debug),
            Mode::Config(config) => config.perform(debug),
        };
    }
}
