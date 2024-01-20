#ifndef GAME_H
#define GAME_H

typedef struct Game {
  int score;
  void (*roll) (struct Game*, int pins);
  int (*get_score) (struct Game*);
} Game;


Game *new_game();
void game_roll(Game* game, int pins);
int game_get_score(Game* game);

#endif // GAME_H
