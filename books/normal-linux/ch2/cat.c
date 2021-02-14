#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

static void do_cat(int fd);
static void die(const char *s);

int main (int argc, char *argv[]) {
    int i;
    int fd = STDIN_FILENO;

    if (argc == 1) do_cat(fd);

    for (i = 1; i < argc; i++) {
        if ((fd = open(argv[i], O_RDONLY)) >= 0) {
            do_cat(fd);
            close(fd);
        } else {
            die(argv[i]);
        }
    }

    exit(0);
}

#define BUFFER_SIZE 2048

static void do_cat(int fd) {
    char buf[BUFFER_SIZE];
    int n;

    while((n = read(fd, buf, sizeof(buf))) != 0) {
        write(STDOUT_FILENO, buf, n);
    }
}

static void die(const char *s) {
    perror(s);
    exit(1);
}
