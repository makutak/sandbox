from Game import Game


class TestBowlingGame:
    def setup_method(self):
        self._game = Game()


    def roll_many(self, n, pins):
        for _ in range(n):
            self._game.roll(pins)

    def test_gutter_game(self):
        self.roll_many(20, 0)

        assert self._game.score() == 0


    def test_all_ones(self):
        self.roll_many(20, 1)

        assert self._game.score() == 20
