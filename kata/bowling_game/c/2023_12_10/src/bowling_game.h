// bowling_game.h
#ifndef BOWLING_GAME_H
#define BOWLING_GAME_H

typedef struct BowlingGame {
  int current_score;
  int rolls[21];
  int current_roll;

  void (*roll)(struct BowlingGame*, int pins);
  int (*score)(struct BowlingGame*);
} BowlingGame;

void bowling_game_roll(BowlingGame* bowling_game, int pins);
int bowling_game_score(BowlingGame *bowling_game);
int sum_of_balls_in_frame();
int spare_bonus(BowlingGame *bowling_game, int frame_index);
int strike_bonus(BowlingGame *bowling_game, int frame_index);
int is_strike(BowlingGame *bowling_game, int frame_index);
int is_spare(BowlingGame *bowling_game, int frame_index);
BowlingGame* new_bowling_game();

#endif  // BOWLING_GAME_H
