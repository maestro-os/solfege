//! Solfège is the default booting system for the Maestro operating system.

//#![deny(warnings)]

mod fstab;
mod module;
mod service;
mod tty;
mod uname;
mod util;

use std::fs;
use std::process::exit;
use std::process::Command;

/// The path to the file containing the startup program.
const STARTUP_PROG_PATH: &str = "/etc/solfege/startup";

/// Runs the startup command.
fn startup() {
	let prog = fs::read_to_string(STARTUP_PROG_PATH).unwrap_or_else(|err| {
		eprintln!("Failed to open startup program configuration file: {}", err);
		exit(1);
	});
	let prog = prog.trim();

	Command::new(prog).spawn().unwrap_or_else(|err| {
		eprintln!("Cannot run startup program: {}", err);
		exit(1);
	});
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
	tty::init().unwrap_or_else(|err| {
		eprintln!("Failed to setup TTY: {}", err);
		exit(1);
	});

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
	module::load_default(&uname).unwrap_or_else(|err| {
		eprintln!("Failed to load default modules: {}", err);
		exit(1);
	});

	println!("Launching services...");
	let mut services_manager = service::Manager::new().unwrap_or_else(|err| {
		eprintln!("Failed to launch the services manager: {}", err);
		exit(1);
	});

	// Running the startup program
	startup();

	loop {
		services_manager.tick();
	}
}
