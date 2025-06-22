mod processer;
mod cli;
mod mode;
mod init;

use std::{fs, io::Result, path::PathBuf};

use clap::Parser;

use cli::Args;
use include_dir::{include_dir, Dir};
use mode::Mode;
use processer::Processer;

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/default_config");

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("{:#?}", args);
    }

    if args.generate_default_config {
        let config = init::get_config_dir();
        if args.debug {
            println!("Attempting to generate default config at {:?}", config);
        }

        assert!(!config.is_dir(), "Configuration directory already exists!");
        fs::create_dir(&config).expect("Couldn't create empty configuration directory!");

        generate_default_config(&config).expect("Couldn't generate default config!");
    }

    match args.mode {
        Mode::Run(run) => run.execute(args.debug),
        Mode::Shell(shell) => shell.execute(args.debug),
        Mode::Develop(develop) => develop.execute(args.debug),
        Mode::Init(init) => init.make(args.debug),
    };
}

/// Tries to copy the embedded default config directory to the destination directory.
fn generate_default_config(dest: &PathBuf) -> Result<()> {
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
