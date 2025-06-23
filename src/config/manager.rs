use std::{collections::HashMap, fs, path::PathBuf};
use std::io;

use serde::Deserialize;

/// Maps the names of available templates to their full paths 
fn map_templates(dir: &PathBuf) -> io::Result<HashMap<String, PathBuf>> {
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

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub shell: String,
}

impl ConfigFile {
    pub fn new(file: &PathBuf) -> Self  {
        assert!(file.is_file(), "This is not a file!");
        let contents = fs::read_to_string(file).expect("Couldn't read config.toml!");
        let config = toml::from_str(&contents).expect("Couldn't parse config.toml!");
        config
    }
}

#[derive(Debug)]
pub struct ConfigManager {
    pub template_dir: PathBuf,
    templates: HashMap<String, PathBuf>,
    pub debug: bool,
    pub config_file: ConfigFile,
}

impl ConfigManager {
    pub fn new (debug: bool) -> Self {
        let config_dir = get_config_dir();
        let template_dir = config_dir.join("templates");
        let templates = map_templates(&template_dir).expect("Couldn't map templates!");
        let config_file = ConfigFile::new(&config_dir.join("config.toml"));

        let config = Self {
            template_dir,
            templates,
            debug,
            config_file,
        };

        if debug {
            println!("{:#?}", config);
        }

        config
    }

    /// Gets a templates full path from its name
    pub fn get_template(&self, name: &String) -> &PathBuf {
        self.templates.get(name).expect("Couldn't find template!")
    }

    // fn read_config(&self) -> 
}
