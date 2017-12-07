from lib import readinput
my_input = readinput(__file__)

from functools import partial

def day_1_x(row_sum, data):
    return sum(row_sum(row) for row in data)

day_1_1 = partial(
    day_1_x,
    lambda row: max(row) - min(row)
)
day_1_2 = partial(
    day_1_x,
    lambda row: next(i // j for i in row for j in row if i > j and i % j == 0)
)

refined_input = [
    [int(n) for n in row.split()]
    for row in my_input.split('\n')
]

print(f'part 1: {day_1_1(refined_input)}')
print(f'part 2: {day_1_2(refined_input)}')
