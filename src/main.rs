//! Solfège is the default booting system for the Maestro operating system.

//#![deny(warnings)]

mod module;
mod service;
mod uname;

use std::path::Path;
use std::process::exit;

fn main() {
    println!("Hello world!");
    let uname = uname::UnameInfo::get().unwrap_or_else(| _ | {
        eprintln!("Cannot retrieve system informations with uname");
        exit(1);
    });
    println!("Booting {} release {}", uname.sysname, uname.release);

    // Loading default modules
    let default_modules_path_str = format!("/lib/modules/maestro-{}/default/", uname.release);
    let default_modules_path = Path::new(&default_modules_path_str);
    module::load_all(&default_modules_path).unwrap_or_else(| err | {
        eprintln!("Failed to load default modules: {}", err);
        exit(1);
    });

    // TODO Init drivers manager

    // TODO Parse /etc/fstab if it exists and mount everything

    // TODO Launch services

    // TODO Launch default program with root

    // TODO Set signal handlers
    // TODO Wait child processes to discard exit codes

    loop {}
}
