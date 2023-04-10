#include <fcntl.h>
#include <sys/syscall.h>
#include <unistd.h>

// Loads a module from the given file descriptor.
//
// On success, the function returns `1`. On error, `0`.
int load_module(int fd)
{
	return syscall(SYS_finit_module, fd, NULL, 0) >= 0;
}
