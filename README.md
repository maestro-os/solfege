# Solfège

Solfège is the Maestro operating system's default booting system.



## Building

The `build.sh` allows to build the program.
The path to the libraries should be specified with `LD_LIBRARY_PATH`.



### Dependencies

The following C libraries are required:
- libc
- libunwind



## Services

A service is a daemon which is launched at system startup if enabled.

Each services are described with a file located in the directory `/etc/solfege/services`.

A service descriptor is a JSON file with the following fields:

- `name`: The service's name
- `description`: The service's description
- `enabled`: Tells whether the service is enabled
- `restart_delay` (optional): The delay in millisecond before restarting the service after a crash. If not specified, the service is not restarted
- `user`: The user used to run the service
- `group`: The group used to run the service
- `start_program`: The path to the program to start the service
- `reload_program` (optional): The path to the program to reload the service
- `stop_program` (optional): The path to the program to stop the service

Example:

```json
{
	"name": "test",
	"description": "A simple test service",
	"enabled": true,
	"restart_delay": 1000,
	"user": "root",
	"group": "root",
	"start_program": "/sbin/test",
	"reload_program": "/sbin/test_reload",
	"stop_program": "/sbin/test_stop",
}
```
