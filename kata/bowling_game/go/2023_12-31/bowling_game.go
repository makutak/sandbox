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
	frameIndex := 0

	for frame := 0; frame < 10; frame++ {
		if g.rolls[frameIndex] == 10 {
			// strike
			score += 10 + g.rolls[frameIndex+1] + g.rolls[frameIndex+2]
			frameIndex++
		} else if g.isSpare(frameIndex) {
			score += 10 + g.rolls[frameIndex+2]
			frameIndex += 2
		} else {
			score += g.rolls[frameIndex] + g.rolls[frameIndex+1]
			frameIndex += 2
		}
	}

	return score
}

func (g *Game) isSpare(frameIndex int) bool {
	return g.rolls[frameIndex]+g.rolls[frameIndex+1] == 10
}
