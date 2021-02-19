#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
  int i;
  int line_count = 0;

  for (i = 1; i < argc; i++) {
    FILE *f;
    int c;

    f = fopen(argv[i], "r");
    if (!f) {
      perror(argv[i]);
      exit(1);
    }

    while((c = fgetc(f))) {
      if (c == '\n') line_count++;

      if (c == EOF) {
        line_count++;
        break;
      }
    }
    fclose(f);
  }

  printf("%d %s\n", line_count, argv[1]);
  exit(0);
}
