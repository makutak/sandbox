#include "Game.h"

void Game::roll(int pins) {
  score += pins;
  rolls[currentRoll++] = pins;
}

int Game::getScore() {
  int score = 0;
  int frameIndex = 0;
  for (int frame = 0; frame < 10; frame++) {
    if (isStrike(frameIndex)) {
      score += strikeBonus(frameIndex);
      frameIndex += 1;
    } else if (isSpare(frameIndex)) {
      score += spareBonus(frameIndex);
      frameIndex += 2;
    } else {
      score += sumOfBallsInFrame(frameIndex);
      frameIndex += 2;
    }
  }

  return score;
}

bool Game::isSpare(int frameIndex) {
  return rolls[frameIndex] + rolls[frameIndex + 1] == 10;
}

bool Game::isStrike(int frameIndex) { return rolls[frameIndex] == 10; }

int Game::sumOfBallsInFrame(int frameIndex) {
  return rolls[frameIndex] + rolls[frameIndex + 1];
}

int Game::spareBonus(int frameIndex) { return 10 + rolls[frameIndex + 2]; }

int Game::strikeBonus(int frameIndex) {
  return 10 + rolls[frameIndex + 1] + rolls[frameIndex + 2];
}
