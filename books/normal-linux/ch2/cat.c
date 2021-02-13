#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>


static void do_cat(FILE *f);
static void die(const char *s);

int main (int argc, char *argv[]) {
    int i;
    FILE *f;

    if (argc == 1) {
        do_cat(stdin);
    }

    for (i = 1; i < argc; i++) {
        f = fopen(argv[i], "r");
        if (f == NULL) die(argv[i]);

        do_cat(f);
        fclose(f);
    }

    exit(0);
}

#define BUFFER_SIZE 2048

static void do_cat(FILE *f) {
    char buf[BUFFER_SIZE];

    while((fgets(buf, sizeof buf, f) != NULL)) {
        printf("%s", buf);
    }
}

static void die(const char *s) {
    perror(s);
    exit(1);
}
