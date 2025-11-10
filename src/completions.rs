use std::io;

use clap::{Args, CommandFactory};
use clap_complete::shells;

use crate::cli::{self, Actionable};

#[derive(Debug, Args)]
pub struct Completions {
    #[arg(value_enum)]
    shell: shells::Shell,
}

impl Actionable for Completions {
    fn perform(&self, dryrun: bool) {
        let mut app = cli::Args::command();
        let mut buf = io::stdout();

        if dryrun {
            println!("Generated completions");
        } else {
            clap_complete::generate(self.shell, &mut app, "nf", &mut buf);
        }
    }
}
