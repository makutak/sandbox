// bowling_game.h
#ifndef BOWLING_GAME_H
#define BOWLING_GAME_H

typedef struct BowlingGame {
  int current_role;
  int current_score;
  void (*roll)(struct BowlingGame*, int pins);
  int (*score)(struct BowlingGame*);
} BowlingGame;

void bowling_game_init();
void bowling_game_roll(BowlingGame* bowling_game, int pins);
int bowling_game_score(BowlingGame* bowling_game);
BowlingGame* new_bowling_game();

#endif  // BOWLING_GAME_H
