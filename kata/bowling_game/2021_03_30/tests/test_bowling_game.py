from src.game import Game
import pytest


class TestBowlingGame:
    def setup_method(self, method):
        self.g = Game()

    def roll_many(self, n, pins):
        for i in range(n):
            self.g.roll(pins)

    def test_gutter_game(self):
        self.roll_many(20, 0)
        assert 0 == self.g.score()

    def test_all_one(self):
        self.roll_many(20, 1)
        assert 20 == self.g.score()

    def test_one_spare(self):
        self.g.roll(5)
        self.g.roll(5)
        self.g.roll(3)
        self.roll_many(17, 0)
        assert 16 == self.g.score()


# 41まで
