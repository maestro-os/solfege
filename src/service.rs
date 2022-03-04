//! This modules handles services.

use crate::util;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io;
use std::process::Child;
use std::process::Command;

/// The path to the services directory.
const SERVICES_PATH: &str = "/etc/solfege/services";

/// The service's state.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ServiceState {
    /// The service is running.
    Running,
    /// The service is stopped.
    Stopped,
    /// The service has crashed.
    Crashed,
}

/// Structure representing a service as a file.
#[derive(serde::Deserialize)]
pub struct ServiceDescriptor {
    /// The service's name.
    name: String,
    /// The service's description.
    description: String,

    /// Tells whether the service is enabled.
    enabled: bool,

    /// The delay in milliseconds before restarting the service after it crashed. If None, the
    /// service won't be restarted automaticaly.
    restart_delay: Option<u64>,

    /// The UID used to run the service.
    uid: u32,
    /// The GID used to run the service.
    gid: u32,

    /// The program to start the service.
    start_program: String,
    /// The program to reload the service.
    reload_program: Option<String>,
    /// The program to stop the service.
    stop_program: Option<String>,
}

/// Structure representing a service.
pub struct Service {
    /// The service as represented in its file.
    desc: ServiceDescriptor,

    /// The current state of the service.
    state: ServiceState,

    /// The service's current process.
    process: Option<Child>,
    /// The timestamp of the last crash.
    crash_timestamp: u64,
}

impl Service {
    /// Tells whether the service is enabled.
    pub fn is_enabled(&self) -> bool {
        self.desc.enabled
    }

    /// Returns current state of the service.
    pub fn get_state(&self) -> ServiceState {
        self.state
    }

    /// Starts the service. If the service is already started, the function does nothing.
    pub fn start(&mut self) -> io::Result<()> {
        if self.state != ServiceState::Running {
            // TODO Use uid and gid
            self.process = Some(Command::new(&self.desc.start_program).spawn()?);
        }

        Ok(())
    }

    /// Reloads the service.
    pub fn reload(&mut self) {
        if self.state != ServiceState::Running {
            self.start();
        } else if let Some(reload_prg) = &self.desc.reload_program {
            Command::new(reload_prg).spawn();
        }
    }

    /// Stops the service. If the service is already stopped, the function does nothing.
    pub fn stop(&mut self) {
        if self.state != ServiceState::Running {
            return;
        }

        if let Some(stop_prg) = &self.desc.stop_program {
            Command::new(stop_prg).spawn();
        }
    }

    /// Restarts the service.
    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }
}

/// Structure representing the services manager.
pub struct Manager {
    /// The list of services.
    services: Vec<Service>,
}

impl Manager {
    /// Reads the list of services.
    fn list() -> io::Result<Vec<Service>> {
        let mut services = Vec::new();

        let e = fs::read_dir(SERVICES_PATH)?;
        for entry in e {
            let e = entry.unwrap();
            let p = e.path();
            let file_type = e.file_type().unwrap();

            if file_type.is_file() {
                let file = File::open(p)?;
                let reader = BufReader::new(file);

                let desc: ServiceDescriptor = serde_json::from_reader(reader)?;
                services.push(Service {
                    desc,

                    state: ServiceState::Stopped,

                    process: None,
                    crash_timestamp: 0,
                });
            }
        }

        Ok(services)
    }

    /// Creates a new instance.
    pub fn new() -> io::Result<Self> {
        let mut services = Self::list()?;

        for s in &mut services {
            if s.is_enabled() {
                s.start();
            }
        }

        Ok(Self {
            services,
        })
    }

    /// Returns an immutable reference to the list of services.
    pub fn get_services(&self) -> &Vec<Service> {
        &self.services
    }

    /// Ticks the manager. This function is used to restart services that died.
    pub fn tick(&mut self) {
        for s in &mut self.services {
            if !s.is_enabled() || s.get_state() != ServiceState::Crashed {
                continue;
            }

            if let Some(delay) = s.desc.restart_delay {
                if util::get_timestamp() >= s.crash_timestamp + delay {
                    s.start();
                }
            }
        }
    }
}
