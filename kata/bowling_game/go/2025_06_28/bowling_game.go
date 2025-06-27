package main

type Game struct {
	totalScore int
}

func NewGame() *Game {
	return &Game{}
}

func (g *Game) Roll(pins int) {

}

func (g *Game) Score() int {
	return 0
}
