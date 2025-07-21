use std::{collections::HashMap, fs, path::PathBuf};
use std::io;

use serde::Deserialize;

/// Maps the names of available templates to their full paths 
pub fn map_templates() -> io::Result<HashMap<String, PathBuf>> {
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

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir().expect("Couldn't get configuration directory!").join("nf")
}

pub fn get_template_dir() -> PathBuf {
    get_config_dir().join("templates")
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub shell: String,
    pub nested_flakes: bool,
}

impl ConfigFile {
    pub fn new() -> anyhow::Result<Self> {
        let file = get_config_dir().join("config.toml");
        let contents = fs::read_to_string(file)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}
