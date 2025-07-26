mod config;
mod expansions;

mod cli;
mod completions;
mod init;
mod mode;
mod nest;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    args.handle();
}
