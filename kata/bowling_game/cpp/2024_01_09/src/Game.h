#ifndef GAME_H
#define GAME_H


class Game {
private:
  int score = 0;

public:
  int getScore();
  void roll(int pins);
};

#endif // GAME_H
