use clap::Parser;

use crate::mode::Mode;

/// This is used recursively to process the argument tree
pub trait Actionable {
    /// Performs the action
    fn perform(&self, dryrun: bool);
}

/// Simple program inspired by nix-helper that allows for fancy nix command expansions.
/// To use certain commands you first need to generate a default configuration using:
/// `nf config create`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    mode: Mode,

    /// Does not execute any commands or make changes to disk. Instead, processer commands (expansions) will return their fully expanded shell cmd to stdout instead of executing it themselves. This can be used for greater control and integration into the interactive shell.
    #[arg(long)]
    dryrun: bool,
}

impl Args {
    pub fn handle(&self) {
        log::debug!("Cli args: {:?}", self);

        self.mode.perform(self.dryrun);
    }
}
