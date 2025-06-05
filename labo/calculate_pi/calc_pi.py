import math
import json

def calc_chord_length(n):
    a_n = 1
    for _ in range(1, n + 1):
        a_n = math.sqrt(2 - math.sqrt(4 - a_n**2))
    return a_n
    
if __name__ == '__main__':
    chords = [calc_chord_length(n) for n in range(11)]
    approximate_pi_list = [6 * (2 ** n) * value / 2 for n, value in enumerate(chords)]
    print(approximate_pi_list)
    dict = {f'{6 * 2**index}角形': value for index, value in enumerate(approximate_pi_list)}
    print(json.dumps(dict, ensure_ascii=False, indent=4))
