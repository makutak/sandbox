class Game:
    def __init__(self):
        self._rolls = [0] * 21
        self._current_role = 0

    def roll(self, pins):
        self._rolls[self._current_role] = pins
        self._current_role += 1

    def score(self):
        score = 0
        frame_index = 0
        for _ in range(10):
            if (self._rolls[frame_index] + self._rolls[frame_index + 1] == 10): # spare
                score += 10 + self._rolls[frame_index + 2]
                frame_index += 2
            else:
                score += self._rolls[frame_index] + self._rolls[frame_index + 1]
                frame_index += 2

        return score
