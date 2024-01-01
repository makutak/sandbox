package bowling

import "testing"

func TestGutterGame(t *testing.T) {
	game := NewGame()
	for i := 0; i < 20; i++ {
		game.Roll(0)
	}
	if game.Score() != 0 {
		t.Errorf("Expected score of 0 but got %d", game.Score())
	}
}

func TestAllOnes(t *testing.T) {
	game := NewGame()
	for i := 0; i < 20; i++ {
		game.Roll(1)
	}
	if game.Score() != 20 {
		t.Errorf("Expected score of 20 but got %d", game.Score())
	}
}