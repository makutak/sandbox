#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

struct cmd {
  int argc;
  char **argv;
  int capa;
  int status;
  int pid;
  struct cmd *next;
};


static void prompt(void);
static struct cmd *parse_command_line(char *cmdline);
static void free_cmd(struct cmd *p);
static void* xmalloc(size_t sz);
static void* xrealloc(void *ptr, size_t sz);

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
  struct cmd  *cmd;

  fprintf(stdout, "$ ") ;
  fflush(stdout);

  if (fgets(buf, LINEBUF_MAX, stdin) == NULL)
    exit(0);

  cmd = parse_command_line(buf);

  if (cmd == NULL) {
    fprintf(stderr, "%s: syntac error\n", program_name);
    return;
  }
}
