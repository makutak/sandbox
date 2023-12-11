// bowling_game.h
#ifndef BOWLING_GAME_H
#define BOWLING_GAME_H

typedef struct BowligGame {
  int roll;
  int score[21];
} BowligGame;

void bowling_game_init();
void bowling_game_roll(int pins);
int bowling_game_score();

#endif  // BOWLING_GAME_H
