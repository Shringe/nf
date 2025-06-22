mod processer;
mod cli;
mod mode;
mod init;
mod config;
mod completions;

use clap::Parser;

use cli::Args;

fn main() {
    let args = Args::parse();
    args.handle();
}
