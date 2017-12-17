from lib import readinput
my_input = readinput(__file__)

def gen_spinlock_passes(steps):
    virtual_len, position = 1, 0
    value = 1
    while True:
        position = (position + steps) % virtual_len + 1
        yield (position, value)
        virtual_len += 1
        value += 1

def day_17_1(spinlock_steps):
    buffer = [0]
    spinlock_passes = gen_spinlock_passes(spinlock_steps)

    for pos, value in spinlock_passes:
        buffer.insert(pos, value)
        if value == 2017:
            return buffer[(pos + 1) % len(buffer)]

def day_17_2(spinlock_steps):
    short_circuit_value = None
    spinlock_passes = gen_spinlock_passes(spinlock_steps)

    for pos, value in spinlock_passes:
        if pos == 1:
            short_circuit_value = value
        if value == 50000000:
            break

    return short_circuit_value

spinlock_steps = int(my_input)

print(f'part 1: {day_17_1(spinlock_steps)}')
print(f'part 2: {day_17_2(spinlock_steps)}')
