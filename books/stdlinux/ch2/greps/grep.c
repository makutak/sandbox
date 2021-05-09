#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <regex.h>
#define _GNU_SOURCE
#include <getopt.h>

static void do_grep(regex_t *pat, FILE *f, int is_ignore);

static struct option longopts[] = {
  {"ignore-case", required_argument, NULL, 'i'},
  {"invert-match", required_argument, NULL, 'v'},
  {0, 0, 0, 0}
};

int main(int argc, char **argv) {
  regex_t re;
  int i;
  int err;
  int opt;
  int is_invert = 0;
  int is_ignore = 0;
  char *pattern;

  // オプションがあれば解析する
  while ((opt = getopt_long(argc, argv, "i:v:", longopts, NULL)) != -1) {
    switch(opt) {
      case 'i':
        is_invert = 1;
        pattern = optarg;
        break;
      case 'v':
        is_ignore = 1;
        pattern = optarg;
        break;
      case '?':
        fprintf(stderr, "Usage: %s [-i | -v PATTERN] [FILE]\n", argv[0]);
        exit(1);
    }
  }

  if (argc < 1) {
    fputs("no pattern\n", stderr);
    exit(1);
  }

  if (!is_ignore && !is_invert) pattern = argv[1];

  int re_mode = REG_EXTENDED | REG_NOSUB | REG_NEWLINE;
  if (is_invert) re_mode |= REG_ICASE;
  err = regcomp(&re, pattern, re_mode);

  if (err != 0) {
    printf("err != 0\n");
    char buf[1024];

    regerror(err, &re, buf, sizeof(buf));
    puts(buf);
    exit(1);
  }

  if (argc == optind) {
    do_grep(&re, stdin, is_ignore);
  } else {
    for (i = optind; i < argc; i++) {
      FILE *f;

      f = fopen(argv[i], "r");
      if (!f) {
        perror(argv[i]);
        exit(1);
      }
      do_grep(&re, f, is_ignore);
      fclose(f);
    }
  }

  regfree(&re);
  exit(0);
}

static void do_grep(regex_t *pat, FILE *src, int is_ignore) {
  char buf[4096];
  int matched;

  while (fgets(buf, sizeof(buf), src)) {
    matched = (regexec(pat, buf, 0, NULL, 0) == 0);
    if (is_ignore == 1) {
      matched = !matched;
    }
    if (matched) {
      fputs(buf, stdout);
    }
  }
}
