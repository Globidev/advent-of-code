from lib import readinput
my_input = readinput(__file__)

from enum import Enum
from collections import namedtuple
from functools import reduce

program_count = 16
initial_line = [chr(i + ord('a')) for i in range(program_count)]
dance_repeat_count = 1000000000

class MoveType(Enum):
    Spin = 1
    Exchange = 2
    Partner = 3

DanceMove = namedtuple('DanceMove', ['type', 'args'])

def apply_n(f, n, i):
    return reduce(lambda s, _: f(s), range(n), i)

move_parsers = {
    's': (MoveType.Spin, int),
    'x': (MoveType.Exchange, int),
    'p': (MoveType.Partner, str),
}

def spin(line, pivot):
    return line[-pivot:] + line[:-pivot]

def exchange(line, a, b):
    new_line = line[:]
    new_line[a], new_line[b] = new_line[b], new_line[a]
    return new_line

def partner(line, a, b):
    return exchange(line, line.index(a), line.index(b))

move_logics = {
    MoveType.Spin: spin,
    MoveType.Exchange: exchange,
    MoveType.Partner: partner,
}

def parse_danse_move(move_str):
    move_type, arg_parser = move_parsers[move_str[0]]
    args = [arg_parser(arg) for arg in move_str[1:].split('/')]

    return DanceMove(move_type, args)

def apply_dance_move(program_line, dance_move):
    move_logic = move_logics[dance_move.type]
    new_program_line = move_logic(program_line, *dance_move.args)

    return new_program_line

def day_16_1(dance_moves):
    final_line = reduce(apply_dance_move, dance_moves, initial_line)

    return ''.join(final_line)

def day_16_2(dance_moves):
    dance = lambda line: reduce(apply_dance_move, dance_moves, line)
    cycle_length, cycle_start = floyd(dance, initial_line)

    # Exact minimum number of dances required to reach the final state
    dance_count = cycle_start + (dance_repeat_count - cycle_start) % cycle_length
    final_line = apply_n(dance, dance_count, initial_line)

    return ''.join(final_line)

def floyd(f, x0):
    tortoise = f(x0)
    hare = f(f(x0))
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(f(hare))

    mu = 0
    tortoise = x0
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(hare)
        mu += 1

    lam = 1
    hare = f(tortoise)
    while tortoise != hare:
        hare = f(hare)
        lam += 1

    return lam, mu

dance_moves = [
    parse_danse_move(move_str)
    for move_str in my_input.split(',')
]

print(f'part 1: {day_16_1(dance_moves)}')
print(f'part 2: {day_16_2(dance_moves)}')
