import unittest
from Game import Game

class TestGame(unittest.TestCase):
    def test_gutter_game(self):
        g = Game()
        for _ in range(0, 20):
            g.roll(0)
        self.assertEqual(0, g.score())

    def test_all_ones(self):
        g = Game()

        for _ in range(0, 20):
            g.roll(1)
        self.assertEqual(20, g.score())


if __name__ == '__main__':
    unittest.main()
