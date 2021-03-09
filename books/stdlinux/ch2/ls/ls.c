#include <stdio.h>
#include <stdlib.h>

static void do_ls(char *path);

int main(int argc, char **argv) {
  int i;

  if (argc < 2) {
    fprintf(stderr, "%s: no arguments.\n", argv[0]);
    exit(1);
  }

  for (i = 1; i < argc; i++) {
    do_ls(argv[i]);
  }

  return 0;
}

static void do_ls(char *path) {
  printf("path: %s\n", path);
}
