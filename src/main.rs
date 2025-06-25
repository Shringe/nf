mod config;
mod expansions;

mod cli;
mod mode;
mod init;
mod completions;

use clap::Parser;

use cli::Args;

fn main() {
    let args = Args::parse();
    args.handle();
}
