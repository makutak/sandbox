#ifndef GAME_H
#define GAME_H

typedef struct Game {
  int (*score) (struct Game*);
} Game;

Game *new_game();
int game_score(Game* game);

#endif // GAME_H
