#include <stdio.h>

static void do_ls();

int main(int argc, char **argv) {
  int i;

  if (argc < 2) {
    fprintf(stderr, "%s: no arguments.\n", argv[0]);
  }
  for (i = 1; i < argc; i++) {
    printf("%s\n", argv[i]);
  }
  return 0;
}
