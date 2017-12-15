from lib import readinput
my_input = readinput(__file__)

gen_a_factor = 16807
gen_b_factor = 48271
gen_a_criteria = 4
gen_b_criteria = 8
gen_modulo = 2147483647
part_1_iterations = 40000000
part_2_iterations = 5000000
mask = 0xFFFF

def generator(seed, factor):
    value = seed
    while True:
        value = (value * factor) % gen_modulo
        yield value

def criteria_generator(seed, factor, criteria):
    gen = generator(seed, factor)

    return (value for value in gen if value % criteria == 0)

def judge_count(gen_a, gen_b, iterations):
    values = ((next(gen_a), next(gen_b)) for _ in range(iterations))
    count = sum((a & mask) == (b & mask) for a, b in values)

    return count

def day_15_1(gen_a_seed, gen_b_seed):
    gen_a = generator(gen_a_seed, gen_a_factor)
    gen_b = generator(gen_b_seed, gen_b_factor)

    return judge_count(gen_a, gen_b, part_1_iterations)

def day_15_2(gen_a_seed, gen_b_seed):
    gen_a = criteria_generator(gen_a_seed, gen_a_factor, gen_a_criteria)
    gen_b = criteria_generator(gen_b_seed, gen_b_factor, gen_b_criteria)

    return judge_count(gen_a, gen_b, part_2_iterations)

gen_a_seed, gen_b_seed = (
    int(line.split()[-1])
    for line in my_input.split('\n')
)

print(f'part 1: {day_15_1(gen_a_seed, gen_b_seed)}')
print(f'part 2: {day_15_2(gen_a_seed, gen_b_seed)}')
