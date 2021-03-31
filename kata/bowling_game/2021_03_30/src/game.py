class Game:
    def __init__(self):
        self.__rolls = [0] * 21
        self.__current_roll = 0

    def roll(self, pins):
        self.__rolls[self.__current_roll] = pins
        self.__current_roll += 1

    def score(self):
        score = 0
        frame_index = 0
        for frame in range(10):
            if self.__is_strike(frame_index):
                score += 10 + self.__strike_bonus(frame_index)
                frame_index += 1
            elif self.__is_spare(frame_index):
                score += 10 + self.__spare_bonus(frame_index)
                frame_index += 2
            else:
                score += self.__sum_of_balls_in_frame(frame_index)
                frame_index += 2

        return score

    def __is_strike(self, frame_index):
        return self.__rolls[frame_index] == 10

    def __sum_of_balls_in_frame(self, frame_index):
        return self.__rolls[frame_index] + self.__rolls[frame_index + 1]

    def __spare_bonus(self, frame_index):
        return self.__rolls[frame_index + 2]

    def __strike_bonus(self, frame_index):
        return self.__rolls[frame_index + 1] + self.__rolls[frame_index + 2]

    def __is_spare(self, frame_index):
        return self.__rolls[frame_index] + self.__rolls[frame_index + 1] == 10
