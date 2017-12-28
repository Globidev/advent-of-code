from lib import readinput, apply_n
my_input = readinput(__file__)

from collections import defaultdict, namedtuple

Blueprint = namedtuple('Blueprint', ['start_state', 'steps', 'instructions'])
Transition = namedtuple('Transition', ['write', 'direction', 'next_state'])
TuringMachine = namedtuple('Machine', ['tape', 'cursor', 'state', 'blueprint'])

def parse_transition(instructions_lines):
    write_str, direction_str, next_state_str = instructions_lines
    write = int(write_str[-2])
    direction = direction_str.split(' ')[-1][:-1]
    next_state = next_state_str[-2]

    return Transition(write, direction, next_state)

def parse_instructions(instructions_lines):
    state_str = instructions_lines[0]
    state = state_str[-2]
    on_0 = parse_transition(instructions_lines[2:5])
    on_1 = parse_transition(instructions_lines[6:9])

    return state, { 0: on_0, 1: on_1 }

def parse_blueprint(blueprint_str):
    start_state_str, steps_str, _, *rest = blueprint_str.split('\n')
    start_state = start_state_str[-2]
    steps = int(steps_str.split(' ')[-2])
    instructions = dict(
        parse_instructions(rest[i:i+10])
        for i in range(0, len(rest), 10)
    )

    return Blueprint(start_state, steps, instructions)

def turing_machine(blueprint):
    return TuringMachine(defaultdict(int), 0, blueprint.start_state, blueprint)

def transition(machine):
    instructions = machine.blueprint.instructions[machine.state]
    current_value = machine.tape[machine.cursor]
    transition = instructions[current_value]

    machine.tape[machine.cursor] = transition.write
    d_cursor = +1 if transition.direction == 'right' else -1

    return TuringMachine(
        machine.tape,
        machine.cursor + d_cursor,
        transition.next_state,
        machine.blueprint
    )

def day_25_1(blueprint):
    initial_machine = turing_machine(blueprint)
    final_machine = apply_n(transition, blueprint.steps, initial_machine)

    return sum(value for value in final_machine.tape.values())

def day_25_2(blueprint):
    pass

blueprint = parse_blueprint(my_input)

print(f'part 1: {day_25_1(blueprint)}')
print(f'part 2: {day_25_2(blueprint)}')
