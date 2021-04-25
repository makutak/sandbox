#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>

static void log_exit(char *fmt, ...);
static void* xmalloc(size_t sz);

int main(int argc, char **argv) { return 0; }


static void log_exit(char *fmt, ...) {
  va_list ap;

  va_start(ap, fmt);
  vprintf(stderr, fmt, ap);
  fputc('\n', stderr);
  va_end(ap);
  exit(1);
}

static void* xmalloc(size_t sz) {
  void *p;

  p = malloc(sz);

  if (!p) log_exit("failed to allocate memory");
  return p;
}
