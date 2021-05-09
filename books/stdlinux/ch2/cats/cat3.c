#include <stdio.h>
#include <stdlib.h>
#define _GNU_SOURCE
#include <getopt.h>

static void do_cat(FILE *fd, int type);
static void die(const char *s);

static struct option longopts[] = {
  {"tab", no_argument, NULL, 't'},
  {"newline", no_argument, NULL, 'n'},
  {0, 0, 0, 0}
};


int main(int argc, char **argv) {
  int opt;
  int i;
  int type = 0;

  while((opt = getopt_long(argc, argv, "tn", longopts, NULL)) != -1) {
    switch (opt) {
      case 't':
        type = opt;
        break;
      case 'n':
        type = opt;
        break;
      case '?':
        fprintf(stderr, "Usage: %s [-t | -n | --tab | --newline] [FILE ...]\n", argv[0]);
        exit(1);
    }
  }

  printf("option: %c\n", type);
  printf("optind: %d\n", optind);

  if (argc == 1) do_cat(stdin, type);

  for (i = optind; i < argc; i++) {
    FILE *f;

    f = fopen(argv[i], "r");
    if (!f) die(argv[i]);

    do_cat(f, type);
    fclose(f);
  }

  exit(0);
}

static void do_cat(FILE *fd, int type) {
  int c;

  while ((c = fgetc(fd)) != EOF) {
    if (c == '\t' && type == 't') {
      if (fputs("\\t", stdout) == EOF) exit(1);
    } else if ( c == '\n' && type == 'n') {
      if (fputs("$\n", stdout) == EOF) exit(1);
    } else {
      if (putchar(c) < 0) exit(1);
    }
  }
}

static void die(const char *s) {
  perror(s);
  exit(1);
}
