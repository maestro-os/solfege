//! Solfège is the default booting system for the Maestro operating system.

//#![deny(warnings)]

mod fstab;
mod module;
mod service;
mod tty;
mod uname;
mod util;

use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

/// The path to the file containing the startup program.
const STARTUP_PROG_PATH: &str = "/etc/solfege/startup";

/// Runs the startup command.
fn startup() {
	let mut prog = std::fs::read_to_string(STARTUP_PROG_PATH).unwrap_or_else(|err| {
		eprintln!("Failed to open startup program configuration file: {}", err);
		exit(1);
	});
	prog = prog.trim().to_string();

	let _ = Command::new(prog).spawn().unwrap_or_else(|err| {
		eprintln!("Cannot run startup program: {}", err);
		exit(1);
	});
}

// TODO Ensure this doesn't interfer with services
/// Clears zombie children processes.
/// This function is necessary because when a process becomes orphan, the kernel gives it to the
/// init process, which shall dispose of it properly.
fn clear_zombies() -> ! {
	loop {
		// Wait on a child without blocking
		unsafe {
			libc::waitpid(-1, null_mut::<libc::c_int>(), 0);
		}
	}
}

fn main() {
	println!("Hello world!");
	uname::set_hostname().unwrap_or_else(|e| {
		eprintln!("Cannot set system's hostname: {}", e);
	});
	let uname = uname::UnameInfo::get().unwrap_or_else(|_| {
		eprintln!("Cannot retrieve system informations with uname");
		exit(1);
	});
	println!(
		"Booting system with {} kernel, release {}",
		uname.sysname, uname.release
	);

	// Initialize TTY
	println!("Initializing current TTY...");
	tty::init();

	// Mounting default filesystems
	println!("Mounting fstab filesystems...");
	let fstab_entries = fstab::parse(None).unwrap_or_else(|err| {
		eprintln!("Failed to read the fstab file: {}", err);
		exit(1);
	});
	for e in fstab_entries {
		println!("Mounting `{}`...", e.get_path());
		e.mount().unwrap_or_else(|err| {
			eprintln!("Failed to mount `{}`: {}", e.get_path(), err);
			exit(1);
		});
	}

	// Loading default modules
	println!("Loading default modules...");
	let default_modules_path_str =
		format!("/lib/modules/{}-{}/default/", uname.sysname, uname.release);
	let default_modules_path = Path::new(&default_modules_path_str);
	module::load_all(&default_modules_path).unwrap_or_else(|err| {
		eprintln!("Failed to load default modules: {}", err);
		exit(1);
	});

	// TODO Init drivers manager

	println!("Launching services...");
	let mut services_manager = service::Manager::new().unwrap_or_else(|err| {
		eprintln!("Failed to launch the services manager: {}", err);
		exit(1);
	});

	// Running the startup command
	startup();

	println!("Ready! :)");

	// TODO Run in another thread to restart dead services: services_manager.tick();

	clear_zombies();
}
