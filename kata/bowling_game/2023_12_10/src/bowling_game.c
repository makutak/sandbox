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
  for (int i = 0; i < 20; i++) {
    score += game->rolls[i];
  }

  return game->current_score;
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
