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
      score += 10 + strike_bonus(game, frame_index);
      frame_index++;
    } else if (is_spare(game, frame_index)) {
      score += 10 + spare_bonus(game, frame_index);
      frame_index += 2;
    } else {
      score += sum_of_balls_in_frame(game, frame_index);
      frame_index += 2;
    }
  }

  return score;
}

int sum_of_balls_in_frame(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index] + game->rolls[frame_index + 1];
}

int spare_bonus(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index + 2];
}

int strike_bonus(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index + 1] + game->rolls[frame_index + 2];
}


int is_strike(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index] == 10;
}

int is_spare(BowlingGame* game, int frame_index) {
  return game->rolls[frame_index] + game->rolls[frame_index + 1] == 10;
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
