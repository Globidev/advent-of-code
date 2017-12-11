from lib import readinput
my_input = readinput(__file__)

from functools import reduce
from collections import namedtuple

def scan(f, seq, init):
    value = init
    yield value

    for x in seq:
        value = f(value, x)
        yield value

CubicPosition = namedtuple('CubicPosition', ['x', 'y', 'z'])

def distance(cube1, cube2):
    return max(
        abs(cube1.x - cube2.x),
        abs(cube1.y - cube2.y),
        abs(cube1.z - cube2.z),
    )

direction_vectors = {
    'n':  (0,  1, -1),
    'ne': (1,  0, -1),
    'se': (1, -1, 0),
    's':  (0, -1, 1),
    'sw': (-1, 0, 1),
    'nw': (-1, 1, 0),
}

def move(position, direction):
    dx, dy, dz = direction_vectors[direction]

    return CubicPosition(
        x=position.x + dx,
        y=position.y + dy,
        z=position.z + dz
    )

def day_11_1(directions):
    initial_position = CubicPosition(x=0, y=0, z=0)
    final_position = reduce(move, directions, initial_position)

    return distance(initial_position, final_position)

def day_11_2(directions):
    initial_position = CubicPosition(x=0, y=0, z=0)
    all_positions = scan(move, directions, initial_position)

    return max(distance(initial_position, pos) for pos in all_positions)

directions = my_input.split(',')

print(f'part 1: {day_11_1(directions)}')
print(f'part 2: {day_11_2(directions)}')
