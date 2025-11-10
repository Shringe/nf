use std::{fs, path::PathBuf};

use clap::{Args, Subcommand};

use crate::cli::Actionable;
use crate::completions::Completions;
use crate::config::manager::get_config_dir;

use super::initialize;

#[derive(Debug, Args)]
struct Add {
    /// Path of the template to add
    template: PathBuf,

    /// Name to add the template under
    name: String,
}

impl Actionable for Add {
    fn perform(&self, dryrun: bool) {
        let dest = get_config_dir().join(&self.name);
        assert!(!dest.is_file(), "That template already exists!");

        if dryrun {
            println!("{:?} -> {:?}", self.template, dest);
        } else {
            fs::copy(&self.template, dest).expect("Couldn't copy template file!");
        }
    }
}

#[derive(Debug, Args)]
struct Remove {
    /// Name of the template to remove
    template: String,
}

impl Actionable for Remove {
    fn perform(&self, dryrun: bool) {
        let target = get_config_dir().join(&self.template);

        if dryrun {
            println!("Deleting template {} at {:?}", self.template, target);
        } else {
            fs::remove_file(&target).expect("Failed to delete template!");
        }
    }
}

#[derive(Debug, Args)]
struct Create;

impl Actionable for Create {
    fn perform(&self, dryrun: bool) {
        initialize::initialize_defaults(dryrun);
    }
}

#[derive(Debug, Args)]
struct Destroy;

impl Actionable for Destroy {
    fn perform(&self, dryrun: bool) {
        initialize::destroy_configuration(dryrun).expect("Failed to destroy configuration.");
    }
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Adds a template to ~/.config/nf/templates
    Add(Add),
    /// Removes a template from ~/.config/nf/templates
    Remove(Remove),
    /// Generates default config, usually at ~/.config/nf
    Create(Create),
    /// Deletes the existing configuration
    Destroy(Destroy),
    /// Generates shell completions
    Completions(Completions),
}

impl Actionable for Action {
    fn perform(&self, dryrun: bool) {
        match self {
            Action::Add(add) => add.perform(dryrun),
            Action::Remove(remove) => remove.perform(dryrun),
            Action::Create(create) => create.perform(dryrun),
            Action::Destroy(destroy) => destroy.perform(dryrun),
            Action::Completions(completions) => completions.perform(dryrun),
        };
    }
}

#[derive(Debug, Args)]
pub struct Config {
    /// Configuration action to perform
    #[command(subcommand)]
    action: Action,
}

impl Actionable for Config {
    fn perform(&self, dryrun: bool) {
        self.action.perform(dryrun);
    }
}
