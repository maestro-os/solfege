//! This module implements a call to uname. Allowing to retrieve various informations.

use libc::c_char;
use std::fs;
use std::io;
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

/// Turns the given buffer into a `CStr`.
fn array_to_string(buf: &[c_char]) -> String {
	buf.iter().map(|b| (*b) as u8 as char).collect()
}

impl UnameInfo {
	/// Returns the uname informations.
	///
	/// If the uname informations cannot be retrieved, the function returns an error.
	pub fn get() -> io::Result<Self> {
		let mut uname_info = unsafe { mem::zeroed() };
		let result = unsafe { libc::uname(&mut uname_info) };
		if result == 0 {
			Ok(UnameInfo {
				sysname: array_to_string(&uname_info.sysname[..]),
				nodename: array_to_string(&uname_info.nodename[..]),
				release: array_to_string(&uname_info.release[..]),
				version: array_to_string(&uname_info.version[..]),
				machine: array_to_string(&uname_info.machine[..]),
			})
		} else {
			Err(io::Error::last_os_error())
		}
	}
}

/// Sets the system's hostname according to the hostname file.
///
/// If the file is not present, the function doesn't do anything.
pub fn set_hostname() -> io::Result<()> {
	// Read hostname file
	let hostname = match fs::read(HOSTNAME_FILE) {
		Ok(h) => h,
		Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
		Err(e) => return Err(e),
	};
	let result = unsafe { libc::sethostname(hostname.as_ptr() as _, hostname.len()) };
	if result == 0 {
		Ok(())
	} else {
		Err(io::Error::last_os_error())
	}
}
