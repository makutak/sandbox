#include <stdio.h>
#include <string.h>

#include "minunit.h"

int tests_run = 0;

int rolls[21];
int current_roll = 0;
void bowling_game_init() {
  // ゲームの初期化
  current_roll = 0;
  memset(rolls, 0, sizeof(rolls));
}

void bowling_game_roll(int pins) { rolls[current_roll++] = pins; }

int bowling_game_score() {
  int score = 0;
  int frame_index = 0;

  for (int frame = 0; frame < 10; frame++) {
    if (rolls[frame_index] + rolls[frame_index + 1] == 10) {
      score += 10 + rolls[frame_index + 2];
      frame_index += 2;
    } else {
      score += rolls[frame_index] + rolls[frame_index + 1];
      frame_index += 2;
    }
  }

  return score;
}

int bowling_score(int rolls[], int n) {
  int score = 0;
  for (int i = 0; i < n; i++) {
    score += rolls[i];
  }

  return score;
}

static char *test_all_gutters() {
  bowling_game_init();
  for (int i = 0; i < 20; i++) {
    bowling_game_roll(0);
  }
  mu_assert("error, score != 0", bowling_game_score() == 0);

  return 0;
}

static char *test_all_ones() {
  bowling_game_init();
  for (int i = 0; i < 20; i++) {
    bowling_game_roll(1);
  }
  mu_assert("error, score != 20", bowling_game_score() == 20);

  return 0;
}

static char *test_spare() {
  bowling_game_init();
  bowling_game_roll(5);
  bowling_game_roll(5);  // spare
  bowling_game_roll(3);
  for (int i = 0; i < 17; i++) {
    bowling_game_roll(0);
  }
  mu_assert("error, score != 16", bowling_game_score() == 16);
  return 0;
}

static char *test_strike() {
  bowling_game_init();
  bowling_game_roll(10);  // ストライク
  bowling_game_roll(3);
  bowling_game_roll(4);
  for (int i = 0; i < 16; i++) {
    bowling_game_roll(0);  // 残りのロールは全てガーター
  }
  mu_assert("error, score != 24", bowling_game_score() == 24);
  return 0;
}

static char *all_tests() {
  mu_run_test(test_all_gutters);
  mu_run_test(test_all_ones);
  mu_run_test(test_spare);
  // mu_run_test(test_strike);
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
