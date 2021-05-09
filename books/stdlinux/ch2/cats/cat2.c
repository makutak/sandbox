#include <stdio.h>
#include <stdlib.h>

static void do_cat(FILE *fd);
static void die(const char *s);

int main(int argc, char **argv) {
  int i;

  if (argc == 1) do_cat(stdin);

  for (i = 1; i < argc; i++) {
    FILE *f;

    f = fopen(argv[i], "r");
    if (!f) die(argv[i]);

    do_cat(f);
    fclose(f);
  }

  exit(0);
}

static void do_cat(FILE *fd) {
  int c;

  while ((c = fgetc(fd)) != EOF) {
    if (putchar(c) < 0) exit(1);
  }
}

static void die(const char *s) {
  perror(s);
  exit(1);
}
