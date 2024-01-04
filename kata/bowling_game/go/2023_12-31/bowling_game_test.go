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

func rollStandardSpare() {
	g.Roll(5)
	g.Roll(5)
}

func rollDifferentSpare() {
	g.Roll(4)
	g.Roll(6)
}

func rollStrike() {
	g.Roll(10)
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
	rollStandardSpare()
	g.Roll(3)
	rollMany(17, 0)

	if g.Score() != 16 {
		t.Errorf("Expected score of 16 but got %d", g.Score())
	}
}

func TestDifferentPinsSpare(t *testing.T) {
	setUp()

	rollDifferentSpare()
	g.Roll(3)
	rollMany(17, 0)

	if g.Score() != 16 {
		t.Errorf("Expected score of 16 but got %d", g.Score())
	}
}

func TestOneStrike(t *testing.T) {
	setUp()

	rollStrike()
	g.Roll(3)
	g.Roll(4)
	rollMany(16, 0)

	if g.Score() != 24 {
		t.Errorf("Expected score of 24 but got %d", g.Score())
	}
}

func TestPerfectGame(t *testing.T) {
	setUp()

	rollMany(12, 10)

	if g.Score() != 300 {
		t.Errorf("Expected score of 300 but got %d", g.Score())
	}
}

func TestRollInvalidPins(t *testing.T) {
	setUp()

	err := g.Roll(-1)
	if err == nil {
		t.Errorf("Expected an error for invalid pins, but got none")
	}

	expectedErrorMessage := "Invalid number of pins: -1"
	if err.Error() != expectedErrorMessage {
		t.Errorf("Unexpected error message: got %v want %v", err.Error(), expectedErrorMessage)
	}
}
