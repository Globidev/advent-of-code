from lib import readinput
my_input = readinput(__file__)

from functools import partial
from itertools import combinations

def day_4_x(is_valid, data):
    return len([line for line in data if is_valid(line.split())])

no_duplicates = lambda words: len(words) == len(set(words))
day_4_1 = partial(day_4_x, no_duplicates)

no_anagrams = lambda words: not any(
    set(a) == set(b)
    for a, b in combinations(words, 2)
)
day_4_2 = partial(day_4_x, no_anagrams)

passphrases = my_input.split('\n')

print(f'part 1: {day_4_1(passphrases)}')
print(f'part 2: {day_4_2(passphrases)}')
