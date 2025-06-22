use std::{collections::HashMap, fs, io::Result, path::PathBuf};

use clap::Args;

use crate::config::get_config_dir;

/// Maps the names of available templates to their full paths 
fn map_templates(dir: PathBuf) -> Result<HashMap<String, PathBuf>> {
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
pub struct Init {
    /// Name of the template file in <config_dir>/templates/
    pub template: String,
}

impl Init {
    fn get_template_dir() -> PathBuf {
        let tdir = get_config_dir().join("templates");
        assert!(tdir.is_dir(), "Couldn't find config/templates directory!");
        tdir
    }

    pub fn make(&self, debug: bool) {
        let tmap = map_templates(Self::get_template_dir()).expect("Couldn't map templates!");
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
