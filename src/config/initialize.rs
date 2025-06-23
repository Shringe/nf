use std::{fs, io::Result, path::PathBuf};

use include_dir::{Dir, DirEntry, File, include_dir};

use crate::config::manager::get_config_dir;

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/config/default");

/// Initializes a default config file
fn init_file(file: &File<'_>, to: &PathBuf) -> Result<()> {
    let file_name = file.path().file_name().unwrap();
    let file_path = to.join(file_name);

    fs::write(&file_path, file.contents())?;
    Ok(())
}

/// Recursively initializes the default config
fn init_recursive(from: &Dir<'_>, to: &PathBuf) -> Result<()> {
    for entry in from.entries() {
        match entry {
            DirEntry::Dir(dir) => {
                let to = to.join(dir.path().file_name().unwrap());
                fs::create_dir(&to)?;
                init_recursive(dir, &to)?;
            },
            DirEntry::File(file) => init_file(&file, &to)?,
        }
    }

    Ok(())
}

/// Creates default configuration on disk
pub fn initialize_defaults(debug: bool) {
    let dest = get_config_dir();

    if debug {
        println!("Attempting to generate default config at {:?}", dest);
        println!("Warning! This command does actually generate the config, even in debug mode.");
    }

    assert!(!dest.is_dir(), "Configuration directory already exists!");
    fs::create_dir(&dest).expect("Couldn't create empty configuration directory!");

    init_recursive(&DEFAULT_CONFIG, &dest).expect("Failed to initialize default configuration!");
}

/// Destroys any existing configuration
pub fn destroy_configuration(debug: bool) -> Result<()> {
    let cdir = get_config_dir();

    if debug {
        println!("Removing all of {:?}", cdir);
    } else {
        fs::remove_dir_all(cdir)?;
    }

    Ok(())
}
