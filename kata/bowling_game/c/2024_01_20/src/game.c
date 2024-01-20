#include "game.h"
#include <stdlib.h>


void game_roll(Game *game, int pins) {
  game->score += pins;
}

int game_get_score(Game *game) {
  return game->score;
}


Game* new_game() {
  Game* game = malloc(sizeof(Game));
  game->score = 0;
  game->get_score = game_get_score;
  game->roll = game_roll;
  return game;
}
