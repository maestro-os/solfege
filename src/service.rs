//! This modules handles services.

use crate::util;
use serde::Deserialize;
use std::io;

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

    /// The command to start the service.
    start_command: String,
    /// The command to reload the service.
    reload_command: Option<String>,
    /// The command to stop the service.
    stop_command: Option<String>,
}

/// Structure representing a service.
pub struct Service {
    /// The service as represented in its file.
    desc: ServiceDescriptor,

    /// The current state of the service.
    state: ServiceState,

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
    pub fn start(&mut self) {
        if self.state == ServiceState::Running {
            return;
        }

        // TODO
        todo!();
    }

    /// Reloads the service.
    pub fn reload(&mut self) {
        if self.state != ServiceState::Running {
            self.start();
        } else {
            // TODO Run the reload command
            todo!();
        }
    }

    /// Stops the service. If the service is already stopped, the function does nothing.
    pub fn stop(&mut self) {
        if self.state != ServiceState::Running {
            return;
        }

        // TODO
        todo!();
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

        // TODO Iterate over services directory
        // TODO Parse service descriptors and build services

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
