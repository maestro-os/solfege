//! This module handles kernel modules management.

use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

extern "C" {
	fn load_module(path: *const u8) -> bool;
}

/// Loads the module at the given path.
/// On fail, the function returns an error.
pub fn load(path: &Path) -> Result<(), String> {
    println!("Loading module {}...", path.display());

	let mut c_path = path.as_os_str().as_bytes().to_vec();
	c_path.push(0);

	let success = unsafe {
		load_module(c_path.as_ptr())
	};
    if !success {
		// TODO Handle error
		todo!();
	}

	Ok(())
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
