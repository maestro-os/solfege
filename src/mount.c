#include <sys/mount.h>

int mount_fs(const char *source, const char *target, const char *filesystemtype,
	unsigned long mountflags, const void *data)
{
	int res = mount(source, target, filesystemtype, mountflags, data);
	return (res >= 0);
}
