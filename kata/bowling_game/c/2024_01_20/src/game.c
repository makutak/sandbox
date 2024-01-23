#include "game.h"
#include <stdlib.h>
#include <string.h>


void game_roll(Game *game, int pins) {
  game->rolls[game->current_roll++] = pins;
}

int game_get_score(Game *game) {
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


Game* new_game() {
  Game* game = malloc(sizeof(Game));
  memset(game->rolls, 0, sizeof(game->rolls));
  game->current_roll = 0;
  game->get_score = game_get_score;
  game->roll = game_roll;
  return game;
}
