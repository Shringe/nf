use std::{fs, path::PathBuf};

use clap::Args;

#[derive(Debug, Args)]
pub struct Init {
    #[arg(long)]
    pub config: PathBuf,

    #[arg(short, long)]
    pub template: String,
}

impl Init {
    pub fn make(&self, debug: bool) {
        let template_dir = self.config.join("templates");
        let template = template_dir.join(&self.template);
        let destination = PathBuf::from("flake.nix");

        if debug {
            println!("{:?} -> {:?}", template, destination);
        }

        assert!(template.is_file(), "This template does not exist!");
        assert!(!destination.is_file(), "flake.nix already exists!");
        fs::copy(&template, &destination).expect("Couldn't copy file!");
    }
}
