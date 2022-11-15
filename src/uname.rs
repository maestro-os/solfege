//! This module implements a call to uname. Allowing to retrieve various informations.

use libc::c_char;
use std::error::Error;
use std::ffi::CStr;
use std::fs;
use std::mem;

/// The path to the hostname file.
const HOSTNAME_FILE: &str = "/etc/hostname";

/// Structure containing the final informations to be returned outside of this module.
#[derive(Debug)]
pub struct UnameInfo {
	/// Operating system name
	pub sysname: String,
	/// Name within "some implementation-defined network"
	pub nodename: String,
	/// Operating system release
	pub release: String,
	/// Operating system version
	pub version: String,
	/// Hardware identifier
	pub machine: String,
}

/// Turns the given buffer into a CStr.
fn to_cstr(buf: &[c_char]) -> &CStr {
	unsafe { CStr::from_ptr(buf.as_ptr()) }
}

impl UnameInfo {
	/// Returns the uname informations.
	/// If the uname informations cannot be retrieved, the function returns an error.
	pub fn get() -> Result<Self, ()> {
		let mut uname_info = unsafe { mem::zeroed() };
		let result = unsafe { libc::uname(&mut uname_info) };

		if result == 0 {
			Ok(UnameInfo {
				sysname: to_cstr(&uname_info.sysname[..])
					.to_string_lossy()
					.into_owned(),
				nodename: to_cstr(&uname_info.nodename[..])
					.to_string_lossy()
					.into_owned(),
				release: to_cstr(&uname_info.release[..])
					.to_string_lossy()
					.into_owned(),
				version: to_cstr(&uname_info.version[..])
					.to_string_lossy()
					.into_owned(),
				machine: to_cstr(&uname_info.machine[..])
					.to_string_lossy()
					.into_owned(),
			})
		} else {
			Err(())
		}
	}
}

/// Sets the system's hostname according to the hostname file.
/// If the file is not present, the function doesn't do anything.
pub fn set_hostname() -> Result<(), Box<dyn Error>> {
	let hostname = fs::read(HOSTNAME_FILE)?;
	unsafe {
		libc::sethostname(hostname.as_ptr() as _, hostname.len());
	}

	Ok(())
}
