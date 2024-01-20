#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>

#include <stdlib.h>

#include "../src/game.h"

void test_all_gutters(void) {
  Game *game = new_game();
  for (int i = 0; i < 20; i++) {
    game->roll(game, 0);
  }

  CU_ASSERT_EQUAL(0, game->get_score(game));
}

void test_all_ones(void) {
  Game *game = new_game();
  for (int i = 0; i < 20; i++ ){
    game->roll(game, 1);
  }

  CU_ASSERT_EQUAL(20, game->get_score(game));
}


int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    CU_add_test(suite, "test of all ones", test_all_ones);
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
