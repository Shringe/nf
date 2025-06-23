use std::io;

use clap::{Args, CommandFactory};
use clap_complete::shells;

use crate::{cli::{self, ActionableConfig}, config::manager::ConfigManager};

#[derive(Debug, Args)]
pub struct Completions {
    #[arg(value_enum)]
    shell: shells::Shell,
}

impl ActionableConfig for Completions {
    fn perform(&self, config: ConfigManager) {
        let mut app = cli::Args::command();
        let mut buf = io::stdout();

        if config.debug {
            println!("Generated completions");
        } else {
            clap_complete::generate(self.shell, &mut app, "nf", &mut buf);
        }
    }
}
