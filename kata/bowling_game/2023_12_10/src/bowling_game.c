// bowling_game.c
#include "bowling_game.h"

#include <stdlib.h>
#include <string.h>

void bowling_game_roll(BowlingGame* game, int pins) {
  game->current_score += pins;
  game->rolls[game->current_roll++] = pins;
}

int bowling_game_score(BowlingGame* game) {
  int score = 0;
  int frame_index = 0;
  for (int frame = 0; frame < 10; frame++) {

    if (is_strike(game, frame_index)) {
      // strike
      score += 10 + game->rolls[frame_index + 1] + game->rolls[frame_index + 2];
      frame_index++;
    } else if (is_spare(game, frame_index)) {
      // spare
      score += 10 + game->rolls[frame_index + 2];
      frame_index += 2;
    } else {
      score += game->rolls[frame_index] + game->rolls[frame_index + 1];
      frame_index += 2;
    }
  }

  return score;
}


int is_strike(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index] == 10;
}

int is_spare(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index] + game->rolls[frame_index] == 10;
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
