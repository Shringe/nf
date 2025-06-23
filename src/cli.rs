use clap::Parser;

use crate::{config::manager::ConfigManager, mode::Mode};

/// This is used recursively to process the argument tree
pub trait Actionable {
    /// Performs the action
    fn perform(&self, debug: bool);
}

/// This is used recursively to process the argument tree
pub trait ActionableConfig {
    /// Performs the action
    fn perform(&self, config: ConfigManager);
}

/// Simple program inspired by nix-helper that allows for fancy nix command expansions.
/// To use certain commands you first need to generate a default configuration using:
/// `nf config create`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    mode: Mode,

    /// Enables extra debug info and does not actually execute any commands or make changes on
    /// disk.
    #[arg(long)]
    debug: bool,
}

impl Args {
    pub fn handle(&self) {
        if self.debug {
            println!("{:#?}", self);
        }

        self.mode.perform(self.debug);
    }
}
