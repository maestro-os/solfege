#include <unistd.h>

void set_pgrp()
{
	tcsetpgrp(STDIN_FILENO, getpgid(0));
}
