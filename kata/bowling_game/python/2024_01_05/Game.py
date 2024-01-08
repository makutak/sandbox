class Game:
    def __init__(self):
        self._rolls = []

    def roll(self, pins):
        self._rolls.append(pins)

    def score(self):
        score = 0
        frame_index = 0
        for _ in range(10):
            if self.is_spare(frame_index):
                score += 10 + self._rolls[frame_index + 2]
                frame_index += 2
            else:
                score += self._rolls[frame_index] + self._rolls[frame_index + 1]
                frame_index += 2

        return score

    def is_spare(self, frame_index):
        return self._rolls[frame_index] + self._rolls[frame_index + 1] == 10
