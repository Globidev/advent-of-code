from lib import readinput
my_input = readinput(__file__)

from functools import partial

def day_5_x(instruction_mutator, instructions):
    mutable_instructions = instructions[:]
    pc = 0
    cycle_count = 0

    while pc >= 0 and pc < len(instructions):
        offset = mutable_instructions[pc]
        mutable_instructions[pc] = instruction_mutator(offset)
        pc += offset
        cycle_count += 1

    return cycle_count

day_5_1 = partial(
    day_5_x,
    lambda offset: offset + 1
)
day_5_2 = partial(
    day_5_x,
    lambda offset: offset - 1 if offset >= 3 else offset + 1
)

program = [int(n) for n in my_input.split('\n')]

print(f'part 1: {day_5_1(program)}')
print(f'part 2: {day_5_2(program)}')
