package bowling

type Game struct {
	rolls       [21]int
	currentRoll int
}

func NewGame() *Game {
	return &Game{
		currentRoll: 0,
	}
}

func (g *Game) Roll(pins int) {
	g.rolls[g.currentRoll] = pins
	g.currentRoll++
}

func (g *Game) Score() int {
	score := 0
	i := 0
	for frame := 0; frame < 10; frame++ {
		if g.rolls[i]+g.rolls[i+1] == 10 {
			score += 10 + g.rolls[i+2]
			i += 2
		} else {
			score += g.rolls[i] + g.rolls[i+1]
			i += 2
		}
	}

	return score
}
