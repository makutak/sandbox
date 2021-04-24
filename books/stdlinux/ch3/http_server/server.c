#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>

static void log_exit(char *fmt, ...);

int main(int argc, char **argv) { return 0; }


static void log_exit(char *fmt, ...) {
  va_list ap;

  va_start(ap, fmt);
  vprintf(stderr, fmt, ap);
  fputc('\n', stderr);
  va_end(ap);
  exit(1);
}
