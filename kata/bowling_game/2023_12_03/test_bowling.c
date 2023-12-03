#include <stdio.h>

#include "minunit.h"

int tests_run = 0;

int bowling_score(int rolls[], int n) {
  int score = 0;
  for (int i = 0; i < n; i++) {
    score += rolls[i];
  }

  return score;
}

static char *test_all_gutters() {
  int rolls[20] = {0};

  mu_assert("error, score != 0", bowling_score(rolls, 20) == 0);
  return 0;
}

static char *test_all_ones() {
  int rolls[20];
  for (int i = 0; i < 20; i++) {
    rolls[i] = 1;
  }

  mu_assert("error, score != 20", bowling_score(rolls, 20) == 20);
  return 0;
}

static char *all_tests() {
  mu_run_test(test_all_gutters);
  mu_run_test(test_all_ones);
  return 0;
}

int main(int argc, char **argv) {
  char *result = all_tests();
  if (result != 0) {
    printf("%s\n", result);
  } else {
    printf("ALL TESTS PASSD\n");
  }
  printf("Tests run: %d\n", tests_run);

  return result != 0;
}
