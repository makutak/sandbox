#include "game.h"
#include <stdlib.h>
#include <string.h>


void game_roll(Game *game, int pins) {
  game->rolls[game->current_roll++] = pins;
}

int game_get_score(Game *game) {
  int score = 0;
  int frame_index = 0;

  for (int frame = 0; frame < 10; frame++) {
    if (game->rolls[frame_index] == 10) {
      score += 10 + game->rolls[frame_index + 1] + game->rolls[frame_index + 2];
      frame_index += 1;
    }
    else if (game->is_spare(game, frame_index)) {
      score += 10 + game->rolls[frame_index + 2];
      frame_index += 2;
    } else {
      score += game->rolls[frame_index] + game->rolls[frame_index + 1];
      frame_index += 2;
    }
  }

  return score;
}

int game_is_spare(Game *game, int frame_index) {
  return game->rolls[frame_index] + game->rolls[frame_index + 1] == 10;
}


Game* new_game() {
  Game* game = malloc(sizeof(Game));
  memset(game->rolls, 0, sizeof(game->rolls));
  game->current_roll = 0;
  game->get_score = game_get_score;
  game->roll = game_roll;
  game->is_spare = game_is_spare;
  return game;
}
