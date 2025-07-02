package main

import "testing"

func TestGutterGame(t *testing.T) {
	g := NewGame()

	for range 20 {
		g.Roll(0)
	}

	if g.Score() != 0 {
		t.Errorf("Expected 0, got %d", g.Score())
	}
}

func TestAllOnes(t *testing.T) {
	g := NewGame()

	for range 20 {
		g.Roll(1)
	}

	if g.Score() != 20 {
		t.Errorf("Expected 20, got %d", g.Score())
	}
}

func TestOneSpare(t *testing.T) {
	g := NewGame()

	g.Roll(5)
	g.Roll(5)
	g.Roll(3)

	for range 17 {
		g.Roll(0)
	}

	if g.Score() != 16 {
		t.Errorf("Expected 16, got %d", g.Score())
	}
}

func TestOneStrike(t *testing.T) {
	g := NewGame()

	g.Roll(10)
	g.Roll(3)
	g.Roll(4)
	for range 16 {
		g.Roll(0)
	}

	if g.Score() != 24 {
		t.Errorf("Expected 24, got %d", g.Score())
	}
}
