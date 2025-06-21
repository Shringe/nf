use clap::Parser;

use crate::mode::Mode;

/// Simple program inspired by nix-helper that allows for fancy nix command expansions.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub mode: Mode,

    #[arg(long)]
    pub debug: bool,
}
