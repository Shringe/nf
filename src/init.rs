use std::{fs, path::PathBuf};

use clap::Args;

use crate::{cli::Actionable, config::manager::ConfigManager};

#[derive(Debug, Args)]
pub struct Init {
    /// Name of the template file in <config_dir>/templates/
    template: String,
}

impl Actionable for Init {
    fn perform(&self, debug: bool) {
        let config = ConfigManager::new(debug);
        let template = config.get_template(&self.template);
        let destination = PathBuf::from("flake.nix");
        assert!(!destination.is_file(), "flake.nix already exists!");

        if debug {
            println!("{:?} -> {:?}", template, destination);
        } else {
            fs::copy(template, destination).expect("Couldn't copy file!");
        }
    }
}
