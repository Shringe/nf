use std::{collections::HashMap, fs, path::PathBuf};
use std::io::Result;

use include_dir::{include_dir, Dir};

use crate::config::initialize;

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/config/default");

/// Maps the names of available templates to their full paths 
fn map_templates(dir: &PathBuf) -> Result<HashMap<String, PathBuf>> {
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

#[derive(Debug)]
pub struct ConfigManager {
    pub config_dir: PathBuf,
    pub template_dir: PathBuf,
    templates: HashMap<String, PathBuf>,
    pub debug: bool,
}

impl ConfigManager {
    pub fn new (debug: bool) -> Self {
        let config_dir = dirs::config_dir().expect("Couldn't get configuration directory!").join("nf");
        let template_dir = config_dir.join("templates");
        let templates = map_templates(&template_dir).expect("Couldn't map templates!");

        let config = Self {
            config_dir,
            template_dir,
            templates,
            debug,
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

    /// Creates default configuration on disk
    pub fn initialize_defaults(&self) {
        let dest = &self.config_dir;

        if self.debug {
            println!("Attempting to generate default config at {:?}", dest);
            println!("Warning! This command does actually generate the config, even in debug mode.");
        }

        assert!(!dest.is_dir(), "Configuration directory already exists!");
        fs::create_dir(&dest).expect("Couldn't create empty configuration directory!");

        initialize::init_recursive(&DEFAULT_CONFIG, &dest);
    }

    /// Destroys any existing configuration
    pub fn destroy_configuration(&self) -> Result<()> {
        let cdir = &self.template_dir;

        if self.debug {
            println!("Removing all of {:?}", cdir);
        } else {
            fs::remove_dir_all(cdir)?;
        }

        Ok(())
    }
}
