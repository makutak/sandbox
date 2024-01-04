from Game import Game


def test_gutter_game():
    g = Game()
    for _ in range(20):
        g.roll(0)

    assert g.score() == 0
