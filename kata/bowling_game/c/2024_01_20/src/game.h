#ifndef GAME_H
#define GAME_H

typedef struct Game {
  int rolls[21];
  int current_roll;
  void (*roll) (struct Game*, int pins);
  int (*get_score) (struct Game*);
  int (*is_spare) (struct Game*, int frame_index);
  int (*is_strike) (struct Game*, int frame_index);
} Game;


Game *new_game();
void game_roll(Game* game, int pins);
int game_get_score(Game *game);
int game_is_spare(Game *game, int frame_index);
int game_is_strike(Game *game, int frame_index);

#endif // GAME_H
