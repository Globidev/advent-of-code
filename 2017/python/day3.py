from lib import readinput
my_input = readinput(__file__)

from collections import namedtuple
from itertools import cycle

Position = namedtuple('Position', ['x', 'y'])
Vector = namedtuple('Vector', ['dx', 'dy'])

def move(pos, vector):
    return Position(pos.x + vector.dx, pos.y + vector.dy)

def manhattan_distance_to_origin(pos):
    return abs(pos.x) + abs(pos.y)

v_up    = Vector(0,  1)
v_down  = Vector(0, -1)
v_left  = Vector(-1, 0)
v_right = Vector(1,  0)

v_up_left    = Vector(-1,  1)
v_up_right   = Vector(1,   1)
v_down_left  = Vector(-1, -1)
v_down_right = Vector(1,  -1)

def gen_snail_positions():
    directions = cycle([v_right, v_up, v_left, v_down])
    position = Position(0, 0)
    steps_for_direction = 1

    while True:
        # The number of steps increases every 2 directions
        for _ in range(2):
            direction = next(directions)
            # Yield each position and move along the current direction
            for _ in range(steps_for_direction):
                yield position
                position = move(position, direction)

        steps_for_direction += 1

def day_1_1(square):
    snail_positions = gen_snail_positions()
    square_position = [next(snail_positions) for _ in range(square)][-1]
    return manhattan_distance_to_origin(square_position)

def day_1_2(target_square_value):
    adjacent_vectors = [
        v_up,      v_down,     v_left,      v_right,
        v_up_left, v_up_right, v_down_left, v_down_right
    ]
    snail_positions = gen_snail_positions()
    square_values = { next(snail_positions): 1 }

    for position in snail_positions:
        adjacent_positions = (
            move(position, vector)
            for vector in adjacent_vectors
        )
        square_value = sum(
            square_values.get(pos, 0)
            for pos in adjacent_positions
        )
        square_values[position] = square_value

        if square_value > target_square_value:
            return square_value

refined_input = int(my_input)

print(f'part 1: {day_1_1(refined_input)}')
print(f'part 2: {day_1_2(refined_input)}')
