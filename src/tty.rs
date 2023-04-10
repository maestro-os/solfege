//! This module implements TTY-related features.

use std::io;

/// Initializes the current TTY.
pub fn init() -> io::Result<()> {
	// Get current PGID
	let pgid = unsafe { libc::getpgid(0) };
	if pgid < 0 {
		return Err(io::Error::last_os_error());
	}

	// Set the TTY's PGRP
	let ret = unsafe { libc::tcsetpgrp(libc::STDIN_FILENO, pgid) };
	if ret == 0 {
		Ok(())
	} else {
		Err(io::Error::last_os_error())
	}
}
