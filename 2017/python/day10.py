from lib import readinput, apply_n
my_input = readinput(__file__)

from functools import reduce
from operator import xor

knot_size = 256
block_size = 16
round_count = 64
lengths_suffix = [17, 31, 73, 47, 23]

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

def day_10_1(raw_lengths):
    lengths = map(int, raw_lengths.split(','))

    final_knot = knot_hash_round(lengths, Knot())
    fst, snd, *_ = final_knot.marks

    return fst * snd

def day_10_2(raw_lengths):
    lengths = [ord(c) for c in raw_lengths] + lengths_suffix

    apply_round = lambda k: knot_hash_round(lengths, k)
    final_knot = apply_n(apply_round, round_count, Knot())

    sparse_hash = final_knot.marks
    dense_hash = (
        reduce(xor, sparse_hash[i:i+block_size])
        for i in range(0, knot_size, block_size)
    )
    final_hash = ''.join(format(n, '02x') for n in dense_hash)

    return final_hash

print(f'part 1: {day_10_1(my_input)}')
print(f'part 2: {day_10_2(my_input)}')
