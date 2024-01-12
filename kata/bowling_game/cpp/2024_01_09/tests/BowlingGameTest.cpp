#include <gtest/gtest.h>

#include "Game.h"

class BowlingGameTest : public ::testing::Test {
protected:
  Game game;

  void rollMany(int n, int pins) {
    for (int i = 0; i < n; i++) {
      game.roll(pins);
    }
  }

  void rollSpare() {
    game.roll(5);
    game.roll(5);
  }
};


TEST_F(BowlingGameTest, AllGutterGame) {
  rollMany(0, 20);

  ASSERT_EQ(0, game.getScore());
}

TEST_F(BowlingGameTest, AllOnes) {
  rollMany(1, 20);

  ASSERT_EQ(20, game.getScore());
}

TEST_F(BowlingGameTest, OneSpare) {
  rollSpare();
  game.roll(3);
  rollMany(17, 0);
  ASSERT_EQ(16, game.getScore());
}
