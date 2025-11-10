use crate::{
    cli::Actionable,
    config,
    expansions::{processer, unprocesser},
    init, nest,
};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// nix run shell expansion
    Run(processer::Run),
    /// nix shell shell expansion
    Shell(processer::Shell),
    /// nix develop shell expansion
    Develop(processer::Develop),
    /// Reverses a shell expansion. This is a good way to explore the expansions' capabilities
    Reverse(unprocesser::UnProcesser),
    /// Copies flake templates from ~/.config/nf/templates/<name> to ./flake.nix
    Init(init::Init),
    /// Manages the config, usually found in ~/.config/nf
    Config(config::command::Config),
    /// Moves ./flake.* -> ./flake/flake.*. Useful to keep CWD out of the Nix store.
    Nest(nest::Nest),
    /// Moves ./flake/flake.* -> ./flake.*. Useful to put CWD back into the Nix store.
    Unnest(nest::UnNest),
}

impl Actionable for Mode {
    fn perform(&self, dryrun: bool) {
        match self {
            Mode::Run(run) => run.perform(dryrun),
            Mode::Shell(shell) => shell.perform(dryrun),
            Mode::Develop(develop) => develop.perform(dryrun),
            Mode::Reverse(reverse) => reverse.perform(dryrun),
            Mode::Init(init) => init.perform(dryrun),
            Mode::Config(config) => config.perform(dryrun),
            Mode::Nest(nest) => nest.perform(dryrun),
            Mode::Unnest(unnest) => unnest.perform(dryrun),
        };
    }
}
