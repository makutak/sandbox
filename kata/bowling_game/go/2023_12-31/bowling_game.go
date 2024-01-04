package bowling

import "fmt"

type Game struct {
	score       int
	rolls       [21]int
	currentRoll int
}

func NewGame() *Game {
	return &Game{
		score:       0,
		currentRoll: 0,
	}
}

func (g *Game) Roll(pins int) error {
	if pins < 0 || 10 < pins {
		return fmt.Errorf("Invalid number of pins: %d", pins)
	}

	g.rolls[g.currentRoll] = pins
	g.currentRoll++
	return nil
}

func (g *Game) Score() int {
	score := 0
	frameIndex := 0

	for frame := 0; frame < 10; frame++ {
		if g.isStrike(frameIndex) {
			score += g.strikeBonus(frameIndex)
			frameIndex++
		} else if g.isSpare(frameIndex) {
			score += g.spareBonus(frameIndex)
			frameIndex += 2
		} else {
			score += g.sumOfBallsInFrame(frameIndex)
			frameIndex += 2
		}
	}

	return score
}

func (g *Game) isStrike(frameIndex int) bool {
	return g.rolls[frameIndex] == 10
}

func (g *Game) isSpare(frameIndex int) bool {
	return g.rolls[frameIndex]+g.rolls[frameIndex+1] == 10
}

func (g *Game) sumOfBallsInFrame(frameIndex int) int {
	return g.rolls[frameIndex] + g.rolls[frameIndex+1]
}

func (g *Game) spareBonus(frameIndex int) int {
	return 10 + g.rolls[frameIndex+2]
}

func (g *Game) strikeBonus(frameIndex int) int {
	return 10 + g.rolls[frameIndex+1] + g.rolls[frameIndex+2]
}
