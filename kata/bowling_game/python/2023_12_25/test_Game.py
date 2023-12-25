import unittest
from Game import Game

class TestGame(unittest.TestCase):
    def setUp(self):
        self.g = Game()

    def test_gutter_game(self):
        for _ in range(20):
            self.g.roll(0)
        self.assertEqual(0, self.g.score())

    def test_all_ones(self):
        for _ in range(20):
            self.g.roll(1)
        self.assertEqual(20, self.g.score())


if __name__ == '__main__':
    unittest.main()
