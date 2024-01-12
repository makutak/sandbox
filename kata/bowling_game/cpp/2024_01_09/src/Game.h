#ifndef GAME_H
#define GAME_H


class Game {
private:
  int score = 0;
  int rolls[21] = {};
  int currentRoll = 0;

  bool isSpare(int frameIndex);
  bool isStrike(int frameIndex);

  int sumOfBallsInFrame(int frameIndex);
  int spareBonus(int frameIndex);
  int strikeBonus(int frameIndex);

public:
  int getScore();
  void roll(int pins);
};

#endif // GAME_H
