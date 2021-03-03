#include <stdio.h>
#include <stdlib.h>

static void do_grep();

int main(int argc, char **argv) {
  int i;

  if (argc < 2) {
    fputs("no pattern\n", stderr);
    exit(0);
  }

  exit(0);
}

static void do_grep() {}
