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

  void rollStandardSpare() {
    game.roll(5);
    game.roll(5);
  }

  void rollSDifferentSpare() {
    game.roll(1);
    game.roll(9);
  }

  void rollStrike() {
    game.roll(10);
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

TEST_F(BowlingGameTest, StandardSpare) {
  rollStandardSpare();
  game.roll(3);
  rollMany(17, 0);
  ASSERT_EQ(16, game.getScore());
}

TEST_F(BowlingGameTest, DifferentSpare) {
  rollSDifferentSpare();
  game.roll(3);
  rollMany(17, 0);
  ASSERT_EQ(16, game.getScore());
}

TEST_F(BowlingGameTest, OneStrike) {
  rollStrike();
  game.roll(3);
  game.roll(4);
  rollMany(16, 0);
  ASSERT_EQ(24, game.getScore());
}
