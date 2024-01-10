#include <gtest/gtest.h>

#include "Game.h"

class BowlingGameTest : public ::testing::Test {
protected:
  Game game;
};


TEST_F(BowlingGameTest, AllGutterGame) {
  for (int i = 0; i < 20; i++) {
    game.roll(0);
  }
  ASSERT_EQ(0, game.score());
}

TEST_F(BowlingGameTest, AllOnes) {
  for (int i = 0; i < 20; i++) {
    game.roll(1);
  }
  ASSERT_EQ(20, game.score());
}
