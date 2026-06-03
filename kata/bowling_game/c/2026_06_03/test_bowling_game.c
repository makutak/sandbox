#include "bowling_game.h"
#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>

#include <CUnit/TestDB.h>
#include <stdlib.h>

void test_gutter_game() {
  struct Game *game = new_game();

  for (int i = 0; i < 20; i++) {
    game->roll(game, 0);
  }
  CU_ASSERT_EQUAL(0, game->score(game));
}

int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);
  if (NULL != suite) {
    CU_add_test(suite, "test gutter game", test_gutter_game);
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
