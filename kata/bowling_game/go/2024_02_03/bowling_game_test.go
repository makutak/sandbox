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

func rollStandardSpare() {
	game.Roll(5)
	game.Roll(5)
}

func rollDifferentSpare() {
	game.Roll(1)
	game.Roll(9)
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

func TestStandardSpare(t *testing.T) {
	setUp()

	rollStandardSpare()
	game.Roll(3)
	rollMany(17, 0)

	if game.Score() != 16 {
		t.Errorf("Expected score of 16, but got %d", game.Score())
	}
}

func TestDifferentSpare(t *testing.T) {
	setUp()

	rollDifferentSpare()
	game.Roll(3)
	rollMany(17, 0)

	if game.Score() != 16 {
		t.Errorf("Expected score of 16, but got %d", game.Score())
	}
}

func TestOneStrike(t *testing.T) {
	setUp()

	game.Roll(10) // TestOneStrike
	game.Roll(3)
	game.Roll(4)
	rollMany(16, 0)

	if game.Score() != 24 {
		t.Errorf("Expected score of 24, but got %d", game.Score())
	}
}
