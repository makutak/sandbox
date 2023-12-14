// bowling_game.c
#include "bowling_game.h"

#include <stdlib.h>

void bowling_game_init() {
  // イニシャライザコード
}

void bowling_game_roll(BowlingGame* game, int pins) {
  game->current_score += pins;
}

int bowling_game_score(BowlingGame* game) { return game->current_score; }

BowlingGame* new_bowling_game() {
  BowlingGame* game = malloc(sizeof(BowlingGame));
  game->current_role = 0;
  game->current_score = 0;
  game->roll = bowling_game_roll;
  game->score = bowling_game_score;
  return game;
}
