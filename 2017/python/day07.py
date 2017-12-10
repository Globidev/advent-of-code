from lib import readinput
my_input = readinput(__file__)

from collections import defaultdict, namedtuple

Node = namedtuple('Node', ['name', 'weight', 'subtowers'])

def parse_program(program_string):
    name, weight_repr, *relation = program_string.split()
    weight = int(weight_repr[1:-1])
    subtowers = tuple(''.join(relation[1:]).split(',')) if relation else ()

    return Node(name, weight, subtowers)

def bottom_program(programs):
    holders = set(program.name for program in programs if program.subtowers)
    subtower_bottoms = set(
        subtower_name for program in programs
        for subtower_name in program.subtowers
    )
    rott_bottoms = holders - subtower_bottoms

    return next(iter(rott_bottoms))

day_7_1 = bottom_program

def day_7_2(programs):
    programs_by_name = dict((program.name, program) for program in programs)

    def tower_weight(program):
        return program.weight + sum(
            tower_weight(programs_by_name[subtower_name])
            for subtower_name in program.subtowers
        )

    def find_imbalanced_program(program, delta=0):
        subtowers = [
            programs_by_name[subtower_name]
            for subtower_name in program.subtowers
        ]
        subtowers_weights = [tower_weight(subtower) for subtower in subtowers]
        unique_weights = set(subtowers_weights)

        if len(unique_weights) == 1:
            # subtowers are balanced, current program is the problem
            return program, delta
        else:
            # We need to go deeper to find the root of the imbalance
            odd_weight = next(
                weight for weight in unique_weights
                if subtowers_weights.count(weight) == 1
            )
            expected_weight = next(
                weight for weight in unique_weights
                if weight != odd_weight
            )
            imbalanced_subtower = next(
                subtower for (subtower, weight)
                in zip(subtowers, subtowers_weights)
                if weight == odd_weight
            )
            delta = expected_weight - odd_weight
            return find_imbalanced_program(imbalanced_subtower, delta)

    bottom = programs_by_name[bottom_program(programs)]
    program, weight_delta = find_imbalanced_program(bottom)

    return program.weight + weight_delta

programs = [parse_program(program_str) for program_str in my_input.split('\n')]

print(f'part 1: {day_7_1(programs)}')
print(f'part 2: {day_7_2(programs)}')
