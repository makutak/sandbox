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
