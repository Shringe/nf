use std::{fs, path::PathBuf};

use include_dir::{Dir, DirEntry, File};

/// Initializes a default config file
pub fn init_file(file: &File<'_>, to: &PathBuf) {
    let file_name = file.path().file_name().unwrap();
    let file_path = to.join(file_name);

    fs::write(&file_path, file.contents()).expect("Couldn't create config file!");
}

/// Recursively initializes the default config
pub fn init_recursive(from: &Dir<'_>, to: &PathBuf) {
    for entry in from.entries() {
        match entry {
            DirEntry::Dir(dir) => {
                let to = to.join(dir.path().file_name().expect("Couldn't get directory name!"));
                fs::create_dir(&to).expect("Couldn't create directory!");
                init_recursive(dir, &to);
            },
            DirEntry::File(file) => init_file(&file, &to),
        }
    }
}
