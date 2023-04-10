# fstab

The `/etc/fstab` file stores the list of filesystems to mount at boot.

Each line of the file is a filesystem to mount, with the following syntax (this is an example):

```
UUID=b66bad31-f4a4-4d83-9b87-3ddd84b79fc2	/         	ext4      	rw,relatime	0 1
```

Each column has the following meaning:
- `file system`: The filesystem to mount. This can either be an UUID (example: `UUID=b66bad31-f4a4-4d83-9b87-3ddd84b79fc2`), a label (example: `LABEL=hello`) or a file (example: `/dev/sda`).
- `dir`: The directory on which the filesystem will be mounted.
- `type`: The filesystem type.
- `options`: Mount options, comma-separated. TODO: document each option
- `dump`: TODO
- `pass`: TODO

Comments can be added. They must start with the `#` character.
