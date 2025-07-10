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

    def test_one_spare(self):
        self.game.roll(5)
        self.game.roll(5)
        self.game.roll(3)
        self._roll_many(17, 0)
        self.assertEqual(16, self.game.score())

    def test_one_strike(self):
        self.game.roll(10)
        self.game.roll(3)
        self.game.roll(4)
        self._roll_many(16, 0)
        self.assertEqual(24, self.game.score())

    def _roll_many(self, n ,pins):
        for _ in range(n):
            self.game.roll(pins)
