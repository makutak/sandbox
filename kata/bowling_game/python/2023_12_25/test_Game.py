import unittest
from Game import Game

class TestGame(unittest.TestCase):
    def setUp(self):
        self.g = Game()

    def roll_many(self, n, pins):
        for _ in range(n):
            self.g.roll(pins)

    def roll_standard_spare(self):
        self.g.roll(5)
        self.g.roll(5)

    def roll_different_pins_spare(self):
        self.g.roll(6)
        self.g.roll(4)

    def roll_strike(self):
        self.g.roll(10)

    def test_gutter_game(self):
        self.roll_many(20, 0)
        self.assertEqual(0, self.g.score())

    def test_all_ones(self):
        self.roll_many(20, 1)
        self.assertEqual(20, self.g.score())

    def test_standard_spare(self):
        self.roll_standard_spare()
        self.g.roll(3)
        self.roll_many(17, 0);
        self.assertEqual(16, self.g.score())

    def test_different_pins_spare(self):
        self.roll_different_pins_spare()
        self.g.roll(3)
        self.roll_many(17, 0);
        self.assertEqual(16, self.g.score())

    def test_one_strike(self):
        self.roll_strike()
        self.g.roll(3)
        self.g.roll(4)
        self.roll_many(16, 0)
        self.assertEqual(24, self.g.score())

    def test_perfect_game(self):
        self.roll_many(12, 10)
        self.assertEqual(300, self.g.score())

if __name__ == '__main__':
    unittest.main()
