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
static int invoke_commands(struct cmd *cmd);
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

#define INIT_ARGV 8
#define IDENT_CHAR_P(c) (!isspace((int)c) && ((c) != '|') && ((c) != '>'))

static struct cmd *parse_command_line(char *p) {
    struct cmd *cmd;


    cmd = xmalloc(sizeof(struct cmd));
    cmd->argc = 0;
    cmd->argv = xmalloc(sizeof(char*) * INIT_ARGV);
    cmd->capa = INIT_ARGV;
    cmd->next = NULL;
    while (*p) {
      while (*p && isspace((int)*p))
        *p++ ='\0';

      if(!IDENT_CHAR_P(*p))
        break;
      if (*p && IDENT_CHAR_P(*p)) {
        if (cmd->capa <= cmd->argc) {
          cmd->capa *= 2;
          cmd->argv = xrealloc(cmd->argv, cmd->capa);
        }
        cmd->argv[cmd->argc] = p;
        cmd->argv++;
      }

      while (*p && IDENT_CHAR_P((*p)))
        p++;
    }

    if (cmd->capa <= cmd->argc) {
      cmd->capa += 1;
      cmd->argv = xrealloc(cmd->argv, cmd->capa);
    }

    cmd->argv[cmd->argc] = NULL;

    if (*p == '|' || *p == '>') {
      if (cmd == NULL || cmd->argc == 0) goto parse_error;
    }

    return cmd;


parse_error:
    if(cmd) free_cmd(cmd);
    return NULL;
}

static void free_cmd(struct cmd *cmd) {
  if (cmd->next != NULL)
    free_cmd(cmd->next);
  free(cmd->argv);
  free(cmd);
}


static void* xmalloc(size_t sz) {
  void *p;

  p = calloc(1, sz);
  if (!p)
    exit(3);

  return p;
}

static void * xrealloc(void *ptr, size_t sz) {
  void *p;

  if (!ptr) return xmalloc(sz);

  p = realloc(ptr, sz);

  if (!p)
    exit(3);

  return p;
}
