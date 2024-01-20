#include "game.h"
#include <stdlib.h>


void game_roll(Game *game, int pins) {}

int game_score(Game *game) {
  return 0;
}


Game* new_game() {
  Game* game = malloc(sizeof(Game));
  game->score = game_score;
  game->roll = game_roll;
  return game;
}
