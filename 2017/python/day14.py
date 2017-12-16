from lib import readinput, scan, apply_n
my_input = readinput(__file__)

from functools import reduce
from operator import xor
from itertools import product

from collections import deque

knot_size = 256
block_size = 16
round_count = 64
lengths_suffix = [17, 31, 73, 47, 23]
grid_height = 128
grid_width = 128

USED = '1'
FREE = '0'

class Knot:
    def __init__(self):
        self.marks = list(range(knot_size))
        self.position = 0
        self.skip_size = 0

def reverse_range(l, start, length):
    size = len(l)

    for i in range(length // 2):
        from_i = (start + i) % size
        to_i = (start + length - i - 1) % size
        l[from_i], l[to_i] = l[to_i], l[from_i]

def twist_knot(knot, length):
    reverse_range(knot.marks, knot.position, length)
    knot.position += length + knot.skip_size
    knot.skip_size += 1

    return knot

def knot_hash_round(lengths, knot):
    return reduce(twist_knot, lengths, knot)

def knot_hash_bin(raw_lengths):
    lengths = [ord(c) for c in raw_lengths] + lengths_suffix

    apply_round = lambda k: knot_hash_round(lengths, k)
    final_knot = apply_n(apply_round, round_count, Knot())

    sparse_hash = final_knot.marks
    dense_hash = (
        reduce(xor, sparse_hash[i:i+block_size])
        for i in range(0, knot_size, block_size)
    )

    binary_digest = ''.join(format(n, '08b') for n in dense_hash)

    return binary_digest

def day_14_1(key_string):
    grid = (
        knot_hash_bin(f'{key_string}-{i}')
        for i in range(grid_height)
    )
    used_squares_count = sum(
        sum(1 if bit == USED else 0 for bit in row)
        for row in grid
    )

    return used_squares_count

def in_bound_and_used(grid, position):
    x, y = position

    if x >= 0 and y >=0 and x < grid_width and y < grid_height:
        return grid[y][x] == USED
    else:
        return False

def adjacent_used_squares(grid, start_position):
    open_set = deque()
    closed_set = set()

    open_set.append(start_position)
    while open_set:
        position = open_set.pop()
        closed_set.add(position)

        yield position
        x, y = position
        for (dx, dy) in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            neighbor_position = (x + dx, y + dy)
            if neighbor_position in closed_set:
                continue
            if in_bound_and_used(grid, neighbor_position):
                open_set.append(neighbor_position)

def day_14_2(key_string):
    grid = [
        knot_hash_bin(f'{key_string}-{i}')
        for i in range(grid_height)
    ]

    marked_positions = set()
    region_count = 0
    for x, y in product(range(grid_width), range(grid_height)):
        if (x, y) in marked_positions or grid[y][x] == FREE:
            continue
        group_positions = set(
            position
            for position in adjacent_used_squares(grid, (x, y))
        )
        marked_positions |= group_positions
        region_count += 1

    return region_count


print(f'part 1: {day_14_1(my_input)}')
print(f'part 2: {day_14_2(my_input)}')
