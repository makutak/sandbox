import unittest
from bowling_game import Game

class BowlingGameTest(unittest.TestCase):
    def setUp(self):
        self.game = Game()

    def test_gutter_game(self):
        for _ in range(20):
            self.game.roll(0)
        self.assertEqual(0, self.game.score())

    def test_all_ones(self):
        for _ in range(20):
            self.game.roll(1)
        self.assertEqual(20, self.game.score())

    def test_one_spare(self):
        self.game.roll(5)
        self.game.roll(5)
        self.game.roll(3)
        for _ in range(17):
            self.game.roll(0)
        self.assertEqual(16, self.game.score())
