use std::{collections::HashMap, fs, io::Result, path::PathBuf};

use clap::{Args, Subcommand};
use include_dir::{include_dir, Dir};

use crate::cli::Actionable;

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/default_config");

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir().map(|dir| dir.join("nf")).expect("Couldn't find configuration directory!")
}

pub fn get_template_dir() -> PathBuf {
    let tdir = get_config_dir().join("templates");
    assert!(tdir.is_dir(), "Couldn't find config/templates directory!");
    tdir
}

/// Maps the names of available templates to their full paths 
pub fn map_templates() -> Result<HashMap<String, PathBuf>> {
    let dir = get_template_dir();
    assert!(dir.is_dir(), "Couldn't find templates directory to map!");

    let mut templates = HashMap::new();
    
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let full_path = entry.path().canonicalize()?;
        
        if full_path.is_file() {
            if let Some(filename) = entry.file_name().to_str() {
                templates.insert(filename.to_string(), full_path);
            }
        }
    }
    
    Ok(templates)
}

#[derive(Debug, Args)]
struct Add {
    /// Path of the template to add
    template: PathBuf,

    /// Name to add the template under
    name: String,
}

impl Actionable for Add {
    fn perform(&self, debug: bool) {
        let cdir = get_config_dir();
        let tdir = cdir.join("templates");
        assert!(tdir.is_dir(), "Couldn't find template configuration directory!");

        let dest = tdir.join(&self.name);
        assert!(!dest.is_file(), "That template already exists!");

        if debug {
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
        let templates = map_templates().expect("Couldn't map templates!");
        let target = templates.get(&self.template).expect("Couldn't find template!");

        if debug {
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
        let dest = get_config_dir();

        if debug {
            println!("Attempting to generate default config at {:?}", dest);
            println!("Warning! This command does actually generate the config, even in debug mode.");
        }

        assert!(!dest.is_dir(), "Configuration directory already exists!");
        fs::create_dir(&dest).expect("Couldn't create empty configuration directory!");

        for d in DEFAULT_CONFIG.dirs() {
            let dir = dest.join(d.path());
            fs::create_dir(&dir).expect("Couldn't create directory!");

            for file in d.files() {
                let file_name = file.path().file_name().expect("Couldn't get file name!");
                let file_path = dir.join(file_name);

                fs::write(&file_path, file.contents()).expect("Couldn't create config file!");
            }
        }
    }
}

#[derive(Debug, Args)]
struct Destroy;

impl Actionable for Destroy {
    fn perform(&self, debug: bool) {
        let cdir = get_config_dir();

        if debug {
            println!("Removing all of {:?}", cdir);
        } else {
            fs::remove_dir_all(cdir).expect("Couldn't remove directory!");
        }
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
}

impl Actionable for Action {
    fn perform(&self, debug: bool) {
        match self {
            Action::Add(add) => add.perform(debug),
            Action::Remove(remove) => remove.perform(debug),
            Action::Create(create) => create.perform(debug),
            Action::Destroy(destroy) => destroy.perform(debug),
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
