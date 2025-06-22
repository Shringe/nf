use std::{fs, path::PathBuf};

use clap::Args;

use crate::{config, cli::Actionable};

#[derive(Debug, Args)]
pub struct Init {
    /// Name of the template file in <config_dir>/templates/
    template: String,
}

impl Actionable for Init {
    fn perform(&self, debug: bool) {
        let tmap = config::map_templates().expect("Couldn't map templates!");
        let target = tmap.get(&self.template).expect("Template doesn't exist!");
        let destination = PathBuf::from("flake.nix");
        assert!(!destination.is_file(), "flake.nix already exists!");

        if debug {
            println!("{:#?}", tmap);
            println!("{:?} -> {:?}", target, destination);
        } else {
            fs::copy(target, destination).expect("Couldn't copy file!");
        }
    }
}
