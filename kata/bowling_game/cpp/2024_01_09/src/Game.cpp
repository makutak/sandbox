#include "Game.h"

void Game::roll(int pins) {
  score += pins;
  rolls[currentRoll++] = pins;
}

int Game::getScore() {
  int score = 0;
  int frameIndex = 0;
  for (int frame = 0; frame < 10; frame++) {
    if (rolls[frameIndex] == 10) {
      score += 10 + rolls[frameIndex + 1] + rolls[frameIndex + 2];
      frameIndex += 1;
    } else if (isSpare(frameIndex)) {
      score += 10 + rolls[frameIndex + 2];
      frameIndex += 2;
    } else {
      score += rolls[frameIndex] + rolls[frameIndex + 1];
      frameIndex += 2;
    }
  }

  return score;
}

bool Game::isSpare(int frameIndex) {
  return rolls[frameIndex] + rolls[frameIndex + 1] == 10;
}
