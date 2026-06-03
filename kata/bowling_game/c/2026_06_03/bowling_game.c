#include "bowling_game.h"
#include <stdlib.h>

void game_roll(struct Game *self, int pins) {
}

int game_score(struct Game *self) {
  return 0;
}

struct Game *new_game() {
  struct Game *game = malloc(sizeof(struct Game));
  game->roll = game_roll;
  game->score = game_score;

  return game;
}
