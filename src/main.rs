//! Solfège is the default booting system for the Maestro operating system.

//#![deny(warnings)]

mod fstab;
mod module;
mod service;
mod uname;
mod util;

use std::path::Path;
use std::process::exit;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

/// Clears zombie children processes.
fn clear_zombies() {
    loop {
        // Wait on a child without blocking
        let ret = unsafe {
            libc::waitpid(-1, null_mut::<libc::c_int>(), libc::WNOHANG)
        };

        // If not process has been waited, stop
        if ret <= 0 {
            break;
        }
    }
}

fn main() {
    println!("Hello world!");
    let uname = uname::UnameInfo::get().unwrap_or_else(| _ | {
        eprintln!("Cannot retrieve system informations with uname");
        exit(1);
    });
    println!("Booting {} release {}", uname.sysname, uname.release);

    // Loading default modules
    println!("Loading modules...");
    let default_modules_path_str = format!("/lib/modules/maestro-{}/default/", uname.release);
    let default_modules_path = Path::new(&default_modules_path_str);
    module::load_all(&default_modules_path).unwrap_or_else(| err | {
        eprintln!("Failed to load default modules: {}", err);
        exit(1);
    });

    // TODO Init drivers manager

    // Mounting default filesystems
    println!("Mounting fstab filesystems...");
    let fstab_entries = fstab::parse(None).unwrap_or_else(| err | {
        eprintln!("Failed to read the fstab file: {}", err);
        exit(1);
    });
    for e in fstab_entries {
        println!("Mounting `{}`...", e.get_path());
        fstab::mount(&e);
    }

    println!("Launching services...");
    let mut services_manager = service::Manager::new().unwrap_or_else(| err | {
        eprintln!("Failed to launch the services manager: {}", err);
        exit(1);
    });

    // TODO Set signal handlers
    // TODO Launch default program with root

    println!("Ready! :)");

    loop {
        clear_zombies();
        services_manager.tick();

        thread::sleep(Duration::from_millis(1));
    }
}
