from lib import readinput
my_input = readinput(__file__)

from itertools import cycle, islice

make_memory_state = tuple

def reallocate_memory(memory_state):
    '''Compute the memory state after a reallocation cycle

    This can be done using pure arithmetic and without mutations
    For a state                             s = [0, 10, 2, 1]
    the number of banks is                  n = 4
    the highest capacity bank is at index   i = 1
    the number of blocks to redistribute is x = 10
    the common block gain is                c = x / n = 2
    the number of leftover blocks is        l = x % n = 2

    We can define the common memory gain as cg  = [c] * n = [2, 2, 2, 2]
    the raw leftover memory gain as         rlg = [1] * l + [0] * (n - l)
                                                = [1, 1, 0, 0]
    and adjusted leftover memory gain as    lg  = rotate_right(rlg, i + 1)
                                                = [0, 0, 1, 1]
    the total memory gain is                tg  = cg + lg = [2, 2, 3, 3]

    Now if we take the state after emptying
    the bank and before redistributing      s' = [0, 0, 2, 1]
    we can simply get the resulting state   r  = s' + tg = [2, 2, 5, 4]
    '''

    bank_count = len(memory_state)
    highest_capacity_bank_index, redistributable_blocks_count = (
        max(enumerate(memory_state), key=lambda p: (p[1], -p[0]))
    )

    # Empty the the highest capacity bank
    blocks_before_distribution = (
        block_count if bank_index != highest_capacity_bank_index else 0
        for bank_index, block_count in enumerate(memory_state)
    )

    common_gain, leftovers = divmod(redistributable_blocks_count, bank_count)
    raw_leftover_blocks = (1,) * leftovers + (0,) * (bank_count - leftovers)

    start_offset = bank_count - (highest_capacity_bank_index + 1)
    leftover_blocks = islice(
        cycle(raw_leftover_blocks),
        start_offset, start_offset + bank_count
    )

    block_gains = list(
        common_gain + leftover_block
        for leftover_block in leftover_blocks
    )

    return make_memory_state(
        block_before_distribution + block_gain
        for block_before_distribution, block_gain in
        zip(blocks_before_distribution, block_gains)
    )

# from https://en.wikipedia.org/wiki/Cycle_detection#Floyd.27s_Tortoise_and_Hare
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

def day_6_1(initial_memory_state):
    cycle_length, cycle_start = floyd(reallocate_memory, initial_memory_state)
    return cycle_start + cycle_length

def day_6_2(initial_memory_state):
    cycle_length, _ = floyd(reallocate_memory, initial_memory_state)
    return cycle_length

initial_memory_state = make_memory_state(int(n) for n in my_input.split())

print(f'part 1: {day_6_1(initial_memory_state)}')
print(f'part 2: {day_6_2(initial_memory_state)}')
