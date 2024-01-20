#include "game.h"
#include <stdlib.h>


int game_score(Game *game) {
  return 0;
}


Game* new_game() {
  Game* game = malloc(sizeof(Game));
  game->score = game_score;

  return game;
}
