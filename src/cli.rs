use clap::Parser;

use crate::mode::Mode;

/// Simple program inspired by nix-helper that allows for fancy nix command expansions.
/// To use certain commands you first need to generate a default configuration using:
/// `nf config --generate-default`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Mode,

    /// Enables extra debug info and does not actually execute any commands or make changes on
    /// disk.
    #[arg(long)]
    pub debug: bool,
}
