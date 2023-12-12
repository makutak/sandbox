// bowling_game.c
#include "bowling_game.h"

#include <stdlib.h>

void bowling_game_init() {
  // イニシャライザコード
}

void bowling_game_roll(int pins) {
  // ピンを倒すコード
}

int bowling_game_score() {
  return 0;
}

BowligGame* new_bowling_game() {
  BowligGame* game = malloc(sizeof(BowligGame));
  game->current_role = 0;
  game->roll = bowling_game_roll;
  game->score = bowling_game_score;
  return game;
}
