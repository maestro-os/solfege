//! This module handles kernel modules management.

use crate::uname::UnameInfo;
use std::ffi::c_int;
use std::fs::File;
use std::fs;
use std::io;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::path::PathBuf;

extern "C" {
	fn load_module(fd: c_int) -> bool;
}

/// Loads the module at the given path.
///
/// On fail, the function returns an error.
pub fn load(path: &Path) -> io::Result<()> {
	println!("Loading module `{}`...", path.display());

	let file = File::open(path)?;

	let success = unsafe { load_module(file.as_raw_fd()) };
	if success {
		Ok(())
	} else {
		Err(io::Error::last_os_error())
	}
}

/// Unloads the module with the given name.
///
/// On fail, the function returns an error.
pub fn unload(_name: &str) -> Result<(), String> {
	// TODO
	todo!();
}

/// Loads every modules recursively in the given directory.
///
/// On success, the function returns the number of modules loaded.
///
/// If the directory doesn't exist, the function returns an error.
pub fn load_all(path: &Path) -> io::Result<()> {
	for entry in fs::read_dir(path)? {
		let e = entry?;
		let p = e.path();
		let file_type = e.file_type()?;

		if file_type.is_dir() {
			load_all(&p)?;
		} else if file_type.is_file() {
			load(&p)?;
		}

		// TODO Handle symlinks?
	}

	Ok(())
}

/// Loads default modules.
pub fn load_default(uname: &UnameInfo) -> io::Result<()> {
	let default_modules_path: PathBuf = format!(
		"/lib/modules/{}-{}/default/",
		uname.sysname,
		uname.release
	).into();

	load_all(&default_modules_path)
}
