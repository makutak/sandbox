#ifndef GAME_H
#define GAME_H

typedef struct Game {
  void (*roll) (struct Game*, int pins);
  int (*score) (struct Game*);
} Game;


Game *new_game();
void game_roll(Game* game, int pins);
int game_score(Game* game);

#endif // GAME_H
