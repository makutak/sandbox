class Game:
    def __init__(self):
        self.score_total = 0

    def roll(self, pins):
        self.score_total += pins

    def score(self):
        return self.score_total
