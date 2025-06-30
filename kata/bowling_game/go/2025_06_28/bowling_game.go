package main

type Game struct {
	rolls []int
}

func NewGame() *Game {
	return &Game{}
}

func (g *Game) Roll(pins int) {
	g.rolls = append(g.rolls, pins)
}

func (g *Game) Score() int {
	score := 0
	frameIndex := 0

	for range 10 {
		if g.rolls[frameIndex]+g.rolls[frameIndex+1] == 10 {
			score += 10 + g.rolls[frameIndex+2]
			frameIndex += 2
		} else {
			score += g.rolls[frameIndex] + g.rolls[frameIndex+1]
			frameIndex += 2
		}
	}

	return score
}
