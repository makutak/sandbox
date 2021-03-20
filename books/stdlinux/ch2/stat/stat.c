#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

int main(int argc, char **argv) {
  struct stat st;

  if (argc != 2) {
    fprintf(stderr, "%s: wrong arguments.\n", argv[0]);
    exit(1);
  }

  if (lstat(argv[1], &st) < 0) {
    perror(argv[1]);
    exit(1);
  }

  printf("type: \t%o\n", (st.st_mode & S_IFMT));
  printf("mode: \t%o\n", st.st_mode & ~S_IFMT);
  exit(0);
}
