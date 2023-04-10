# Services

A service is a daemon which is launched at system startup if enabled.



## Service descriptor

Each service is described with a file, located in the directory `/etc/solfege/services`.

Each service descriptor is in TOML format. Example:

```toml
name = "example"
desc = "Just an example service"
enabled = true
restart_delay = 10 # optional
user = "root"
group = "root"
program_path = "/path/to/program"
```

- `name` is the name of the service.
- `desc` is the description of the service.
- `enabled` tells whether the service is enabled.
- `restart_delay` is the delay in milliseconds to wait before restarting the service after it exited. If not specified, the service is never restarted.
- `user` is the name of the user owning the process.
- `group` is the name of the group owning the process.
- `program_path` is the path of the program to run.
