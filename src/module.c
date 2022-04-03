#include <fcntl.h>
#include <sys/syscall.h>
#include <unistd.h>

// Loads the module at path `path`.
// On success, the function returns `1`. On error, `0`.
int load_module(const char *path)
{
	int fd = open(path, O_RDONLY);
	if (fd < 0)
		return 0;

	int success = (syscall(SYS_finit_module, fd, NULL, 0) >= 0);
	close(fd);
	return success;
}
