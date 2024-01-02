package bowling

import (
	"os"
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

func TestMain(m *testing.M) {
	setUp()

	os.Exit(m.Run())
}

func TestGutterGame(t *testing.T) {
	rollMany(20, 0)
	if g.Score() != 0 {
		t.Errorf("Expected score of 0 but got %d", g.Score())
	}
}

func TestAllOnes(t *testing.T) {
	rollMany(20, 1)

	if g.Score() != 20 {
		t.Errorf("Expected score of 20 but got %d", g.Score())
	}
}
