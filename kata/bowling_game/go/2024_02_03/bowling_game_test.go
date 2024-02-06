package bowling

import "testing"

var game *Game

func setUp() {
	game = NewGame()
}

func rollMany(n int, pins int) {
	for i := 0; i < n; i++ {
		game.Roll(pins)
	}
}

func TestGutterGame(t *testing.T) {
	setUp()

	rollMany(20, 0)

	if game.Score() != 0 {
		t.Errorf("Expected score of 0, but got %d", game.Score())
	}
}

func TestAllOnes(t *testing.T) {
	setUp()

	rollMany(20, 1)

	if game.Score() != 20 {
		t.Errorf("Expected score of 20, but got %d", game.Score())
	}
}
