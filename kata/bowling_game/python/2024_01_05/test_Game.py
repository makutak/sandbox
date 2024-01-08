from Game import Game


class TestBowlingGame:
    def setup_method(self):
        self._game = Game()

    def roll_many(self, n, pins):
        for _ in range(n):
            self._game.roll(pins)

    def roll_standard_spare(self):
        self._game.roll(5)
        self._game.roll(5)

    def roll_different_spare(self):
        self._game.roll(9)
        self._game.roll(1)

    def roll_strike(self):
        self._game.roll(10)

    def test_gutter_game(self):
        self.roll_many(20, 0)

        assert self._game.score() == 0

    def test_all_ones(self):
        self.roll_many(20, 1)

        assert self._game.score() == 20

    def test_standard_spare(self):
        self.roll_standard_spare()
        self._game.roll(3)
        self.roll_many(17, 0)

        assert self._game.score() == 16

    def test_different_spare(self):
        self.roll_different_spare()
        self._game.roll(3)
        self.roll_many(17, 0)

        assert self._game.score() == 16

    def test_one_strike(self):
        self.roll_strike()
        self._game.roll(3)
        self._game.roll(4)
        self.roll_many(16, 0)

        assert self._game.score() == 24

    def test_perfect_game(self):
        self.roll_many(12, 10)

        assert self._game.score() == 300
