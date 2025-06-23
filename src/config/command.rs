use std::{fs, path::PathBuf};

use clap::{Args, Subcommand};

use crate::cli::Actionable;
use crate::completions::Completions;

use super::initialize;
use super::manager::ConfigManager;

#[derive(Debug, Args)]
struct Add {
    /// Path of the template to add
    template: PathBuf,

    /// Name to add the template under
    name: String,
}

impl Actionable for Add {
    fn perform(&self, debug: bool) {
        let config = ConfigManager::new(debug);
        let dest = config.template_dir.join(&self.name);
        assert!(!dest.is_file(), "That template already exists!");

        if config.debug {
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
    fn perform(&self, debug: bool) {
        let config = ConfigManager::new(debug);
        let target = config.get_template(&self.template);

        if config.debug {
            println!("Deleting template {} at {:?}", self.template, target);
        } else {
            fs::remove_file(&target).expect("Failed to delete template!");
        }
    }
}

#[derive(Debug, Args)]
struct Create;

impl Actionable for Create {
    fn perform(&self, debug: bool) {
        initialize::initialize_defaults(debug);
    }
}

#[derive(Debug, Args)]
struct Destroy;

impl Actionable for Destroy {
    fn perform(&self, debug: bool) {
        initialize::destroy_configuration(debug).expect("Failed to destroy configuration.");
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
    fn perform(&self, debug: bool) {
        match self {
            Action::Add(add) => add.perform(debug),
            Action::Remove(remove) => remove.perform(debug),
            Action::Create(create) => create.perform(debug),
            Action::Destroy(destroy) => destroy.perform(debug),
            Action::Completions(completions) => completions.perform(debug),
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
    fn perform(&self, debug: bool) {
        self.action.perform(debug);
    }
}
