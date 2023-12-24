import unittest
from Game import Game

class TestGame(unittest.TestCase):
    def test_gutter_game(self):
        g = Game()
        for _ in range(0, 20):
            g.roll(0)
        self.assertEquals(0, g.score())


if __name__ == '__main__':
    unittest.main()
    
