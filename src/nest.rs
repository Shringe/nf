use std::{fs, path::Path};

use clap::Args;

use crate::cli::Actionable;

#[derive(Debug, Args)]
pub struct Nest;

impl Actionable for Nest {
    fn perform(&self, dryrun: bool) {
        let destination = Path::new("flake");
        let flake_from = Path::new("flake.nix");
        let lock_from = Path::new("flake.lock");
        let flake_to = destination.join(flake_from);
        let lock_to = destination.join(lock_from);

        log::debug!("Creating dir: {}", destination.display());
        if !dryrun {
            fs::create_dir("flake").expect("Failed to create ./flake");
        }

        if flake_from.exists() {
            log::debug!("{} -> {}", flake_from.display(), flake_to.display());
            if !dryrun {
                fs::rename(flake_from, flake_to).expect("Failed to move flake.nix");
            }
        }

        if lock_from.exists() {
            log::debug!("{} -> {}", lock_from.display(), lock_to.display());
            if !dryrun {
                fs::rename(lock_from, lock_to).expect("Failed to move flake.lock");
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct UnNest;

impl Actionable for UnNest {
    fn perform(&self, dryrun: bool) {
        let source = Path::new("flake");
        let flake_to = Path::new("flake.nix");
        let lock_to = Path::new("flake.lock");
        let flake_from = source.join(flake_to);
        let lock_from = source.join(lock_to);

        if flake_to.exists() {
            log::debug!("{} -> {}", flake_to.display(), flake_from.display());
            if !dryrun {
                fs::rename(flake_to, flake_from).expect("Failed to move flake.nix");
            }
        }

        if lock_to.exists() {
            log::debug!("{} -> {}", lock_to.display(), lock_from.display());
            if !dryrun {
                fs::rename(lock_to, lock_from).expect("Failed to move flake.lock");
            }
        }

        log::debug!("Removing dir: {}", source.display());
        if !dryrun {
            fs::remove_dir(source).expect("Failed to remove ./flake");
        }
    }
}
