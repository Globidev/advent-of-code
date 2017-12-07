from lib import readinput
my_input = readinput(__file__)

from functools import partial
from itertools import combinations

def day_1_x(is_valid, data):
    return len([line for line in data if is_valid(line.split())])

no_duplicates = lambda words: len(words) == len(set(words))
day_1_1 = partial(day_1_x, no_duplicates)

no_anagrams = lambda words: not any(
    set(a) == set(b)
    for a, b in combinations(words, 2)
)
day_1_2 = partial(day_1_x, no_anagrams)

refined_input = my_input.split('\n')

print(f'part 1: {day_1_1(refined_input)}')
print(f'part 2: {day_1_2(refined_input)}')
