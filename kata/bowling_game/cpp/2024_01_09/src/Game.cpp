#include "Game.h"

void Game::roll(int pins) {
  score += pins;
  rolls[currentRoll++] = pins;
}

int Game::getScore() {
  int score = 0;
  int i = 0;
  for (int frame = 0; frame < 10; frame++) {
    if (rolls[i] + rolls[i + 1] == 10) {
      score += 10 + rolls[i + 2];
      i += 2;
    } else {
      score += rolls[i] + rolls[i + 1];
      i += 2;
    }
  }
  return score;
}
