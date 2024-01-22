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
  game->roll(game, 5);
  game->roll(game, 5); // spare
  game->roll(game, 3);
  roll_many(17, 0);

  CU_ASSERT_EQUAL(16, game->get_score(game));
}

void test_different_one_spare(void) {
  set_up();
  game->roll(game, 2);
  game->roll(game, 8); // spare
  game->roll(game, 3);
  roll_many(17, 0);

  CU_ASSERT_EQUAL(16, game->get_score(game));
}


int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    CU_add_test(suite, "test of all ones", test_all_ones);
    CU_add_test(suite, "test of standard spare", test_standard_one_spare);
    CU_add_test(suite, "test of different spare", test_different_one_spare);
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
