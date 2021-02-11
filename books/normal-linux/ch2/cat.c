#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>


static void do_cat(const char *path);
static void die(const char *s);

int main (int argc, char *argv[]) {
    int i;

    if (argc < 2) {
        fprintf(stderr, "%s: file name not give \n", argv[0]);
    }

    for (i = 1; i < argc; i++) {
        printf("argv[%d]=%s\n", i, argv[1]);
        do_cat(argv[i]);
    }

    exit(0);
}

#define BUFFER_SIZE 2048

static void do_cat(const char *path) {
    int fd;
    unsigned char buf[BUFFER_SIZE];
    int n;

    printf("%s\n", path);
    fd = open(path, O_RDONLY);
    printf("%d\n", fd);
    if (fd < 0) die(path);

    for (;;) {
        n = read(fd, buf, sizeof buf);
        if (n < 0) die(path);
        if (n == 0) break;
        if (write(STDOUT_FILENO, buf, n) < 0) die(path);
    }
    if (close(fd) < 0 ) die(path);
}

static void die(const char *s) {
    perror(s);
    exit(1);
}
