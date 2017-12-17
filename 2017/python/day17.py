from lib import readinput
my_input = readinput(__file__)

from itertools import count

part_1_pass_count = 2017
part_2_pass_count = 50000000

def spinlock_passes(steps):
    position = 0

    for value in count(1):
        buffer_len = value # The buffer size increases with each value
        position = (position + steps) % buffer_len + 1
        yield (position, value)

def day_17_1(spinlock_steps):
    buffer = [0]

    for pos, value in spinlock_passes(spinlock_steps):
        buffer.insert(pos, value)
        if value == part_1_pass_count:
            return buffer[(pos + 1) % len(buffer)]

def day_17_2(spinlock_steps):
    short_circuit_value = None

    for pos, value in spinlock_passes(spinlock_steps):
        if pos == 1: # Inserted after 0
            short_circuit_value = value
        if value == part_2_pass_count:
            return short_circuit_value

spinlock_steps = int(my_input)

print(f'part 1: {day_17_1(spinlock_steps)}')
print(f'part 2: {day_17_2(spinlock_steps)}')
