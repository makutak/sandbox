#ifndef GAME_H
#define GAME_H

typedef struct Game {
  void (*roll) (struct Game*, int pins);
  int (*score) (struct Game*);
} Game;

Game* new_game();

#endif // GAME_H
