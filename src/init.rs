use std::{collections::HashMap, fs, path::PathBuf, process::exit};
use anyhow::Result;
use clap::Args;
use crate::{cli::Actionable, config::manager::map_templates};

/// Recursively gets the full path of every file in a path
fn recursive_read_dir(base: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            out.extend(recursive_read_dir(&path)?);
        }

        out.push(path);
    }

    Ok(out)
}

/// Returns a HashMap of paths "from -> to" for initializing a template
fn map_operations(template: &PathBuf) -> Result<HashMap<PathBuf, PathBuf>> {
   let mut out = HashMap::new();

   for full in recursive_read_dir(&template)? {
       let relative = full.strip_prefix(template)?;
       // if relative.is_file() {
           out.insert(full.clone(), relative.to_path_buf());
       // }
   }

   Ok(out)
}

/// Return any entries about to be copied to CWD if they are already present
fn obstructed_inits(operations: &HashMap<PathBuf,PathBuf>) -> Result<Vec<&PathBuf>> {
   let mut out = Vec::new();

   for to in operations.values() {
       if to.exists() {
           out.push(to);
       }
   }

   Ok(out)
}

/// Initializes the template, overwriting anything in its way
fn initialize_template(operations: &HashMap<PathBuf,PathBuf>) -> Result<()> {
    for (from, to) in operations.iter() {
        if from.is_dir() && !to.is_dir() {
            fs::create_dir_all(to)?;
        }
    }

    for (from, to) in operations.iter() {
        if from.is_file() {
            fs::copy(from, to)?;
        }
    }

    Ok(())
}


#[derive(Debug, Args)]
pub struct Init {
    /// Name of the template file in <config_dir>/templates/
    template: String,

    /// Whether to overwrite files in the CWD with those pulled by the template
    #[arg(long, default_value_t=false)]
    force: bool,
}

impl Actionable for Init {
    fn perform(&self, debug: bool) {
        let templates = map_templates().expect("Couldn't map templates!");
        let template = templates.get(&self.template).expect("Template not found!");

        let operations = map_operations(&template).expect("Couldn't map template initialization!");

        let obstructions = obstructed_inits(&operations).expect("Couldn't handle obstructions!");
        obstructions.iter().for_each(|o| println!("{:?} already exists!", o));

        if !self.force && !obstructions.is_empty() {
            println!("The template couldn't be initialized because some files already exist. Pass --force to initialize anyway, overwriting conflicting files.");
            exit(1);
        }

        if debug {
            dbg!(&templates);
            dbg!(&operations);
            dbg!(&obstructions);
        } else {
            // cp -r $temeplate/* ./
            initialize_template(&operations).unwrap();
        }
    }
}
