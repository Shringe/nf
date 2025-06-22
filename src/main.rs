mod processer;
mod cli;
mod mode;
mod init;

use clap::Parser;

use cli::Args;
use mode::Mode;
use processer::Processer;

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("{:#?}", args);
    }

    match args.mode {
        Mode::Run(run) => run.execute(args.debug),
        Mode::Shell(shell) => shell.execute(args.debug),
        Mode::Develop(develop) => develop.execute(args.debug),
        Mode::Init(init) => init.make(args.debug),
    };
}
