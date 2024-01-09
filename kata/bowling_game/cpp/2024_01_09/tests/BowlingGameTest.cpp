#include <gtest/gtest.h>

#include "Game.h"

TEST(BowlingGameTest, AllGutterGame) {
  Game game;
  for (int i = 0; i < 20; i++) {
    game.roll(0);
  }
  ASSERT_EQ(0, game.score());
}
