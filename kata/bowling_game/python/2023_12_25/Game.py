class Game:
    def __init__(self):
        self._rolls = [0] * 21
        self._current_role = 0

    def roll(self, pins):
        self._rolls[self._current_role] = pins
        self._current_role += 1

    def score(self):
        score = 0
        i = 0
        for _ in range(10):
            if (self._rolls[i] + self._rolls[i + 1] == 10): # spare
                score += 10 + self._rolls[i + 2]
                i += 2
            else:
                score += self._rolls[i] + self._rolls[i + 1]
                i += 2

        return score
