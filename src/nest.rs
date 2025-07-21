use std::{fs, path::Path};

use clap::Args;

use crate::cli::Actionable;

#[derive(Debug, Args)]
pub struct Nest;

impl Actionable for Nest {
    fn perform(&self, debug: bool) {
        if debug {
            println!("mkdir ./flake");

            if Path::new("flake.nix").exists() {
                println!("flake.nix -> flake/flake.nix");
            }

            if Path::new("flake.lock").exists() {
                println!("flake.lock -> flake/flake.lock");
            }
        } else {
            fs::create_dir("flake").expect("Failed to create ./flake!");

            if Path::new("flake.nix").exists() {
                fs::rename("flake.nix", "flake/flake.nix").expect("Failed to move flake.nix");
            }

            if Path::new("flake.lock").exists() {
                fs::rename("flake.lock", "flake/flake.lock").expect("Failed to move flake.lock");
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct UnNest;

impl Actionable for UnNest {
    fn perform(&self, debug: bool) {
        if debug {
            if Path::new("flake/flake.nix").exists() {
                println!("flake/flake.nix -> flake.nix");
            }
            
            if Path::new("flake/flake.lock").exists() {
                println!("flake/flake.lock -> flake.lock");
            }
            
            println!("rmdir ./flake");
        } else {
            if Path::new("flake/flake.nix").exists() {
                fs::rename("flake/flake.nix", "flake.nix").expect("Failed to move flake.nix");
            }
            
            if Path::new("flake/flake.lock").exists() {
                fs::rename("flake/flake.lock", "flake.lock").expect("Failed to move flake.lock");
            }
            
            fs::remove_dir("flake").expect("Failed to remove ./flake!");
        }
    }
}
