#ifndef BOWLING_GAME_H
#define BOWLING_GAME_H

struct Game {
  void (*roll)(struct Game *self, int pins);
  int (*score)(struct Game *self);
};

struct Game *new_game();
void game_roll(struct Game *self, int pins);
int game_score(struct Game *self);

#endif
