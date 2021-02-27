#include <stdio.h>
#include <stdlib.h>

static void do_tail(FILE *fd);

int main(int argc, char **argv) {
  int i;
  FILE *f;

  if (argc != 2) {
    fprintf(stderr, "Usage: %s [FILE]\n", argv[0]);
    exit(1);
  }

  f = fopen(argv[1], "r");
  do_tail(f);

  return 0;
}

void do_tail(FILE *fd) {
  printf("do_tail\n");
  exit(0);
}
