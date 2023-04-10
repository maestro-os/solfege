# Kernel modules

At boot, Solfège loads all the default kernel modules located in `/lib/modules/<kernel>-<release>/default`.

- `<kernel>` is the name of the kernel (`uname -s`)
- `<release>` is the kernel's release (`uname -r`)

If a symbolic link is present, it is followed.
