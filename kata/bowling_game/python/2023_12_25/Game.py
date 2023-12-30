class Game:
    def __init__(self):
        self._rolls = [0] * 21
        self._current_roll = 0

    def roll(self, pins):
        self._rolls[self._current_roll] = pins
        self._current_roll += 1

    def score(self):
        score = 0
        frame_index = 0
        for _ in range(10):
            if (self.is_strike(frame_index)):
                score += 10 + self.strike_bonus(frame_index)
                frame_index += 1
            elif (self.is_spare(frame_index)):
                score += 10 + self.spare_bonus(frame_index)
                frame_index += 2
            else:
                score += self.sum_of_balls_in_frame(frame_index)
                frame_index += 2
        return score

    def is_spare(self, frame_index):
        return self._rolls[frame_index] + self._rolls[frame_index + 1] == 10

    def is_strike(self, frame_index):
        return self._rolls[frame_index] == 10

    def sum_of_balls_in_frame(self, frame_index):
        return self._rolls[frame_index] + self._rolls[frame_index + 1]

    def spare_bonus(self, frame_index):
        return self._rolls[frame_index + 2]

    def strike_bonus(self, frame_index):
        return self._rolls[frame_index + 1] + self._rolls[frame_index + 2]
