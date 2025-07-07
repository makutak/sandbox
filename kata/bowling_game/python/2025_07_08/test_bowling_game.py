import unittest

from bowling_game import Game

class BowlingGameTest(unittest.TestCase):
    def setUp(self):
        self.game = Game()

    def test_gutter_game(self):
        self._roll_many(20, 0)
        self.assertEqual(0, self.game.score())

    def test_all_one(self):
        self._roll_many(20, 1)
        self.assertEqual(20, self.game.score())

    def _roll_many(self, n ,pins):
        for _ in range(n):
            self.game.roll(pins)
