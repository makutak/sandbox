#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>

#include "bowling_game.h"

void test_all_gutters(void) {
  BowligGame *bowling_game = new_bowling_game();

  for (int i = 0; i < 20; i++) {
    bowling_game->roll(0);
  }
  CU_ASSERT(bowling_game->score() == 0);
}

int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    // 他のテストも同様に追加
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
