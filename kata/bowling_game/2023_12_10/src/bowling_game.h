// bowling_game.h
#ifndef BOWLING_GAME_H
#define BOWLING_GAME_H

typedef struct BowligGame {
  int current_role;
  int scores[21];
  void (*roll)(int pins);
  int (*score)();
} BowligGame;

void bowling_game_init();
void bowling_game_roll(int pins);
int bowling_game_score();
BowligGame* new_bowling_game();

#endif  // BOWLING_GAME_H
