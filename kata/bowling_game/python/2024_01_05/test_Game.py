from Game import Game


class TestBowlingGame:
    def setup_method(self):
        self._game = Game()


    def test_gutter_game(self):
        for _ in range(20):
            self._game.roll(0)

        assert self._game.score() == 0


    def test_all_ones(self):
        for _ in range(20):
            self._game.roll(1)

        assert self._game.score() == 20
