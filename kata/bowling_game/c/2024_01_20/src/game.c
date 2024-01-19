#include "game.h"
#include <stdlib.h>

Game* new_game() {
  Game* game = malloc(sizeof(Game));
  return game;
}
