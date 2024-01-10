#include "Game.h"

void Game::roll(int pins) {
  score += pins;
}

int Game::getScore() {
  return score;
}
