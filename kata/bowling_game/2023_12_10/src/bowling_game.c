// bowling_game.c
#include "bowling_game.h"

#include <stdlib.h>
#include <string.h>

void bowling_game_init() {
  // イニシャライザコード
}

void bowling_game_roll(BowlingGame* game, int pins) {
  game->current_score += pins;
  game->rolls[game->current_roll++] = pins;
}

int bowling_game_score(BowlingGame* game) {
  int score = 0;
  int i = 0;
  for (int frame = 0; frame < 10; frame++) {
    if (game->rolls[i] + game->rolls[i + 1] == 10) {
      score += 10 + game->rolls[i + 2];
      i += 2;
    } else {
      score += game->rolls[i] + game->rolls[i + 1];
      i += 2;
    }
  }

  return score;
}

BowlingGame* new_bowling_game() {
  BowlingGame* game = malloc(sizeof(BowlingGame));
  game->current_roll = 0;
  memset(game->rolls, 0, sizeof(game->rolls));
  game->current_score = 0;
  game->roll = bowling_game_roll;
  game->score = bowling_game_score;
  return game;
}
