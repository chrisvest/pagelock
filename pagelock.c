
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/mman.h>

void err(char** argv) {
	printf("Usage: %s <page count>\n", argv[0]);
	exit(1);
}

int main(int argc, char** argv) {
	if (argc != 2) {
		err(argv);
	}
	long long count = strtoll(argv[1], NULL, 10);
	if (!count) {
		err(argv);
	}

	int psize = sysconf(_SC_PAGESIZE);
	size_t len = count * psize;

#ifdef __APPLE__
	void* pointer = mmap(NULL, len, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
#else
	int fd = open("/dev/zero", O_RDWR);
	void* pointer = mmap(NULL, len, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);
#endif

	if (pointer == MAP_FAILED) {
		perror("mmap");
		exit(2);
	}
	if (mlock(pointer, len)) {
		perror("mlock");
		exit(3);
	}
	for (size_t i = 0; i < count; i++) {
		((char*)pointer)[i * psize] = 1;
	}

	printf("Locked %lld pages of %d bytes\n", count, psize);
	printf("Press return to stop...");
	getchar();

	if (munmap(pointer, len)) {
		perror("munmap");
		exit(4);
	}

#ifndef __APPLE__
	close(fd);
#endif

	return 0;
}
