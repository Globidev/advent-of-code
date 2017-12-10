from lib import readinput
my_input = readinput(__file__)

from collections import defaultdict, namedtuple
from functools import reduce

def scan(f, seq, init):
    value = init
    yield value

    for x in seq:
        value = f(value, x)
        yield value

Instruction = namedtuple('Instruction', ['register', 'mutation', 'condition'])
Condition = namedtuple('Condition', ['register', 'comparison', 'value'])

mut_ops = {
    'dec': lambda x: lambda v: v - x,
    'inc': lambda x: lambda v: v + x,
}

comp_ops = {
    '<': lambda a, b: a < b,
    '>': lambda a, b: a > b,
    '<=': lambda a, b: a <= b,
    '>=': lambda a, b: a >= b,
    '==': lambda a, b: a == b,
    '!=': lambda a, b: a != b,
}

make_registers = lambda: defaultdict(int)

def parse_instruction(instruction_str):
    mut_reg, mut_op_str, mut_value, _, comp_reg, comp_op_str, comp_value = (
        instruction_str.split()
    )
    comp_op = comp_ops[comp_op_str]
    condition = Condition(comp_reg, comp_op, int(comp_value))

    mut_op = mut_ops[mut_op_str](int(mut_value))
    instruction = Instruction(mut_reg, mut_op, condition)

    return instruction

def apply_instruction(registers, instruction):
    test_register, compare, test_value = instruction.condition

    if compare(registers[test_register], test_value):
        mutable_register = instruction.register
        new_value = instruction.mutation(registers[mutable_register])
        registers[mutable_register] = new_value

    return registers

def day_8_1(instructions):
    initial_registers = make_registers()
    final_registers = reduce(apply_instruction, instructions, initial_registers)

    return max(final_registers.values())

def day_8_2(instructions):
    initial_registers = make_registers()
    all_register_states = scan(apply_instruction, instructions, initial_registers)

    return max(
        max(registers.values()) for registers in all_register_states
        if registers
    )

instructions = [
    parse_instruction(instruction_str) for
    instruction_str in my_input.split('\n')
]

print(f'part 1: {day_8_1(instructions)}')
print(f'part 2: {day_8_2(instructions)}')
