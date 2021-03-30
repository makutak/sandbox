class Game:
    def __init__(self):
        self.__rolls = [0] * 21
        self.__current_roll = 0

    def roll(self, pins):
        self.__rolls[self.__current_roll] = pins
        self.__current_roll += 1

    def score(self):
        score = 0
        i = 0
        for frame in range(10):
            score += self.__rolls[i] + self.__rolls[i + 1]
            i += 2

        print(score)
        return score
