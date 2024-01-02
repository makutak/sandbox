package bowling

import (
	"testing"
)

var g *Game

func setUp() {
	g = NewGame()
}

func rollMany(n int, pins int) {
	for i := 0; i < n; i++ {
		g.Roll(pins)
	}
}

func TestGutterGame(t *testing.T) {
	setUp()

	rollMany(20, 0)
	if g.Score() != 0 {
		t.Errorf("Expected score of 0 but got %d", g.Score())
	}
}

func TestAllOnes(t *testing.T) {
	setUp()
	rollMany(20, 1)

	if g.Score() != 20 {
		t.Errorf("Expected score of 20 but got %d", g.Score())
	}
}

func TestStandardSpare(t *testing.T) {
	setUp()

	g.Roll(5)
	g.Roll(5) // spare
	g.Roll(3)
	rollMany(17, 0)

	if g.Score() != 16 {
		t.Errorf("Expected score of 16 but got %d", g.Score())
	}
}

func TestDifferentPinsSpare(t *testing.T) {
	setUp()

	g.Roll(4)
	g.Roll(6) // spare
	g.Roll(3)
	rollMany(17, 0)

	if g.Score() != 16 {
		t.Errorf("Expected score of 16 but got %d", g.Score())
	}
}
