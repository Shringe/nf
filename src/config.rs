use std::{fs, io::Result, path::PathBuf};

use clap::Args;
use include_dir::{include_dir, Dir};

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/default_config");

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir().map(|dir| dir.join("nf")).expect("Couldn't find configuration directory!")
}

#[derive(Debug, Args)]
pub struct Config {
    /// Generates the default config directory and contents if it doesn't exist.
    #[arg(long)]
    pub generate_default: bool,
}

impl Config {
    /// Tries to copy the embedded default config directory to the destination directory.
    fn generate_default(debug: bool) -> Result<()> {
        let dest = get_config_dir();

        if debug {
            println!("Attempting to generate default config at {:?}", dest);
        }

        assert!(!dest.is_dir(), "Configuration directory already exists!");
        fs::create_dir(&dest).expect("Couldn't create empty configuration directory!");

        for d in DEFAULT_CONFIG.dirs() {
            let dir = dest.join(d.path());
            fs::create_dir(&dir)?;

            for file in d.files() {
                let file_name = file.path().file_name().expect("Couldn't get file name!");
                let file_path = dir.join(file_name);

                fs::write(&file_path, file.contents())?;
            }
        }

        Ok(())
    }
    
    pub fn process(&self, debug: bool) {
        if self.generate_default {
            Self::generate_default(debug).expect("Couldn't generate default configuration!");
        }
    }
}
