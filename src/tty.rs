//! This module implements TTY-related features.

extern "C" {
	/// Sets the TTY's pgrp with the current process's pgid.
	fn set_pgrp();
}

/// Initializes the current TTY.
pub fn init() {
	unsafe {
		set_pgrp();
	}
}
