//! This module handles kernel modules management.

use std::fs;
use std::path::Path;

/// Loads the module at the given path.
/// On fail, the function returns an error.
pub fn load(path: &Path) -> Result<(), String> {
    println!("Loading module {}...", path.display());

    // TODO
    todo!();
}

/// Unloads the module with the given name.
/// On fail, the function returns an error.
pub fn unload(name: &String) -> Result<(), String> {
    println!("Unloading module {}...", name);

    // TODO
    todo!();
}

/// Loads every modules recursively in the given directory.
/// If the directory doesn't exist, the function returns an error.
pub fn load_all(path: &Path) -> Result<(), String> {
    let e = fs::read_dir(path)
        .or_else(| _ | Err(format!("Failed to open directory `{}`", path.display())))?;

    for entry in e {
        let e = entry.unwrap();
        let p = e.path();
        let file_type = e.file_type().unwrap();

        if file_type.is_dir() {
            load_all(&p)?;
        } else if file_type.is_file() {
            load(&p)?;
        }

        // TODO Handle symlinks?
    }

    Ok(())
}
