//! This modules handles services.

use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::ptr::null_mut;

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
#[derive(Deserialize)]
pub struct ServiceDescriptor {
	/// The service's name.
	name: String,
	/// The service's description.
	desc: String,

	/// Tells whether the service is enabled.
	enabled: bool,

	/// The delay in milliseconds before restarting the service after it crashed.
	///
	/// If `None`, the service won't be restarted automaticaly.
	restart_delay: Option<u64>,

	/// The user used to run the service.
	user: String,
	/// The group used to run the service.
	group: String,

	/// The path to the program of the service.
	program_path: PathBuf,
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

impl From<ServiceDescriptor> for Service {
	fn from(desc: ServiceDescriptor) -> Self {
		Self {
			desc,

			state: ServiceState::Stopped,

			process: None,
			crash_timestamp: 0,
		}
	}
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
			println!("Starting service `{}`...", self.desc.name);

			// TODO Use uid and gid
			let process = Command::new(&self.desc.program_path)
				.spawn()?;

			self.process = Some(process);
			self.state = ServiceState::Running;
		}

		Ok(())
	}

	/// Reloads the service.
	pub fn reload(&mut self) -> io::Result<()> {
		if self.state == ServiceState::Running {
			self.stop()?;
		}

		self.start()
	}

	/// Stops the service. If the service is already stopped, the function does nothing.
	pub fn stop(&mut self) -> io::Result<()> {
		if let Some(ref mut process) = self.process {
			process.kill()?;
			self.process = None;
		}

		self.state = ServiceState::Stopped;

		Ok(())
	}

	/// Restarts the service.
	pub fn restart(&mut self) -> io::Result<()> {
		self.stop()?;
		self.start()?;

		Ok(())
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
			let e = entry?;
			let p = e.path();
			let file_type = e.file_type()?;

			if file_type.is_file() {
				let content = fs::read_to_string(p)?;

				match toml::from_str::<ServiceDescriptor>(&content) {
					Ok(desc) => services.push(Service::from(desc)),
					Err(_e) => {
						// TODO
						todo!();
					},
				};
			}
		}

		Ok(services)
	}

	/// Creates a new instance.
	pub fn new() -> io::Result<Self> {
		let mut services = Self::list()?;

		for s in &mut services {
			if s.is_enabled() {
				s.start()?;
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

	/// Returns the service with the given PID.
	pub fn get_service(&mut self, pid: u32) -> Option<&mut Service> {
		self.services.iter_mut()
			.filter(|s| s.process.as_ref().map(|c| c.id()) == Some(pid))
			.next()
	}

	/// Ticks the manager.
	///
	/// This function is used to restart services that died.
	pub fn tick(&mut self) {
		let pid = unsafe {
			libc::waitpid(-1, null_mut::<libc::c_int>(), 0)
		};
		if pid < 0 {
			// TODO sleep?
			return;
		}

		let Some(_service) = self.get_service(pid as u32) else {
			// An orphan process has been assigned to the current process, then died
			return;
		};

		// TODO update process state
	}
}
