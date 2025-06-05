#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>

#include <stdlib.h>

#include "../src/game.h"

Game *game;

void set_up() { game = new_game(); }

void roll_many(int n, int pins) {
  for (int i = 0; i < n; i++) {
    game->roll(game, pins);
  }
}

void roll_standard_spare() {
  game->roll(game, 5);
  game->roll(game, 5);
}

void roll_different_spre() {
  game->roll(game, 2);
  game->roll(game, 8);
}

void roll_strike() {
  game->roll(game, 10);
}

void test_all_gutters(void) {
  set_up();
  roll_many(20, 0);

  CU_ASSERT_EQUAL(0, game->get_score(game));
}

void test_all_ones(void) {
  set_up();
  roll_many(20, 1);

  CU_ASSERT_EQUAL(20, game->get_score(game));
}

void test_standard_one_spare(void) {
  set_up();
  roll_standard_spare();
  game->roll(game, 3);
  roll_many(17, 0);

  CU_ASSERT_EQUAL(16, game->get_score(game));
}

void test_different_one_spare(void) {
  set_up();
  roll_different_spre();
  game->roll(game, 3);
  roll_many(17, 0);

  CU_ASSERT_EQUAL(16, game->get_score(game));
}

void test_one_strike(void) {
  set_up();
  roll_strike();
  game->roll(game, 3);
  game->roll(game, 4);
  roll_many(16, 0);

  CU_ASSERT_EQUAL(24, game->get_score(game));
}

void test_perfect_game(void) {
  set_up();
  roll_many(12, 10);

  CU_ASSERT_EQUAL(game->get_score(game), 300);
}


int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    CU_add_test(suite, "test of all ones", test_all_ones);
    CU_add_test(suite, "test of standard spare", test_standard_one_spare);
    CU_add_test(suite, "test of different spare", test_different_one_spare);
    CU_add_test(suite, "test of one strike", test_one_strike);
    CU_add_test(suite, "test of perfect game", test_perfect_game);
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
