#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
#include <stdlib.h>

#include "bowling_game.h"

BowlingGame *set_up() { return new_bowling_game(); }
void clean_up(BowlingGame *game) { free(game); }

void test_all_gutters(void) {
  BowlingGame *bowling_game = set_up();

  for (int i = 0; i < 20; i++) {
    bowling_game->roll(bowling_game, 0);
  }
  CU_ASSERT(bowling_game->score(bowling_game) == 0);

  clean_up(bowling_game);
}

void test_all_ones(void) {
  BowlingGame *bowling_game = set_up();

  for (int i = 0; i < 20; i++) {
    bowling_game->roll(bowling_game, 1);
  }
  CU_ASSERT(bowling_game->score(bowling_game) == 20);

  clean_up(bowling_game);
}

int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    CU_add_test(suite, "test of all ones", test_all_ones);
    // 他のテストも同様に追加
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
