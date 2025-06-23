use std::{fs, path::PathBuf};

use clap::{Args, Subcommand};

use crate::cli::{Actionable, ActionableConfig};
use crate::completions::Completions;

use super::manager::ConfigManager;

#[derive(Debug, Args)]
struct Add {
    /// Path of the template to add
    template: PathBuf,

    /// Name to add the template under
    name: String,
}

impl ActionableConfig for Add {
    fn perform(&self, config: ConfigManager) {
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

impl ActionableConfig for Remove {
    fn perform(&self, config: ConfigManager) {
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

impl ActionableConfig for Create {
    fn perform(&self, config: ConfigManager) {
        config.initialize_defaults();
    }
}

#[derive(Debug, Args)]
struct Destroy;

impl ActionableConfig for Destroy {
    fn perform(&self, config: ConfigManager) {
        config.destroy_configuration().expect("Failed to destroy configuration.");
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

impl ActionableConfig for Action {
    fn perform(&self, config: ConfigManager) {
        match self {
            Action::Add(add) => add.perform(config),
            Action::Remove(remove) => remove.perform(config),
            Action::Create(create) => create.perform(config),
            Action::Destroy(destroy) => destroy.perform(config),
            Action::Completions(completions) => completions.perform(config),
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
        let config = ConfigManager::new(debug);
        self.action.perform(config);
    }
}
