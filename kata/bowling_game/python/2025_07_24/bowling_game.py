class Game:
    def __init__(self):
        self.total_score = 0

    def roll(self, pins):
        self.total_score += pins

    def score(self):
        return self.total_score
