package bowling

type Game struct {
	score       int
	rolls       [21]int
	currentRole int
}

func NewGame() *Game {
	return &Game{
		score:       0,
		currentRole: 0,
	}
}

func (g *Game) Roll(pins int) {
	g.rolls[g.currentRole] = pins
	g.currentRole++
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
