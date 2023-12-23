#include <CUnit/Basic.h>
#include <CUnit/CUnit.h>
#include <stdlib.h>

#include "bowling_game.h"

BowlingGame *g;

void set_up() { g = new_bowling_game(); }

void clean_up(BowlingGame *game) { free(game); }

void roll_many(int n, int pins) {
  for (int i = 0; i < n; i++) {
    g->roll(g, pins);
  }
}

void roll_spare() {
  g->roll(g, 5);
  g->roll(g, 5);
}

void roll_strike() {
  g->roll(g, 10);
}

void test_all_gutters(void) {
  set_up();

  roll_many(20, 0);
  CU_ASSERT(g->score(g) == 0);

  clean_up(g);
}

void test_all_ones(void) {
  set_up();

  roll_many(20, 1);
  CU_ASSERT(g->score(g) == 20);

  clean_up(g);
}

void test_one_spare() {
  set_up();

  roll_spare();
  g->roll(g, 3);
  roll_many(17, 0);

  CU_ASSERT(g->score(g) == 16);

  clean_up(g);
}

void test_one_strike() {
  set_up();

  roll_strike();
  g->roll(g, 3);
  g->roll(g, 4);
  roll_many(16, 0);

  CU_ASSERT(g->score(g) == 24);

  clean_up(g);
}

void test_perfect_game() {
  set_up();

  roll_many(12, 10);
  CU_ASSERT(g->score(g) == 300);

  clean_up(g);
}

int main() {
  CU_initialize_registry();
  CU_pSuite suite = CU_add_suite("BowlingGameTest", NULL, NULL);

  if (NULL != suite) {
    CU_add_test(suite, "test of all gutters", test_all_gutters);
    CU_add_test(suite, "test of all ones", test_all_ones);
    CU_add_test(suite, "test of one spare", test_one_spare);
    CU_add_test(suite, "test of one strike", test_one_strike);
    CU_add_test(suite, "test of perfect game", test_perfect_game);
  }

  CU_basic_set_mode(CU_BRM_VERBOSE);
  CU_basic_run_tests();
  CU_cleanup_registry();

  return CU_get_error();
}
