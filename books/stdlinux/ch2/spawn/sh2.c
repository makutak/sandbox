#include <stdio.h>
#include <stdlib.h>


static void prompt(void);

static char *program_name;

int main(int argc, char **argv) {
  program_name = argv[0];

  for (;;) {
    prompt();
  }

  exit(0);
}

#define LINEBUF_MAX 2048

static void prompt(void) {
  static char buf[LINEBUF_MAX];

  fprintf(stdout, "$ ") ;
  fflush(stdout);

  if (fgets(buf, LINEBUF_MAX, stdin) == NULL)
    exit(0);
}
