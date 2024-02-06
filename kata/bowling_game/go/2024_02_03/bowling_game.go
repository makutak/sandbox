package bowling

type Game struct {
	score int
}

func NewGame() *Game {
	return &Game{
		score: 0,
	}
}

func (g *Game) Roll(pins int) {
	g.score += pins
}

func (g *Game) Score() int {
	return g.score
}
