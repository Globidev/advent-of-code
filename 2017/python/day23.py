from lib import readinput
my_input = readinput(__file__)

from collections import defaultdict

def instruction(f):

    def run(program, *tail_args):
        f(program, *tail_args)
        program.pc += 1

    return lambda *args: lambda p: run(p, *args)

def i_set(program, x, y): program.regs[x] = program.value(y)
def i_add(program, x, y): program.regs[x] += program.value(y)
def i_sub(program, x, y): program.regs[x] -= program.value(y)
def i_mod(program, x, y): program.regs[x] %= program.value(y)

def i_mul(program, x, y):
    program.regs[x] *= program.value(y)
    program.mul_count += 1

def i_jnz(program, x, y):
    if program.value(x) != 0:
        program.pc += program.value(y) - 1

class Program:

    def __init__(self):
        self.pc = 0
        self.regs = defaultdict(int)
        self.mul_count = 0
        self.stopped = False

    def value(self, value_or_reg):
        try:
            return int(value_or_reg)
        except ValueError:
            return self.regs[value_or_reg]

    def stop(self):
        self.stopped = True

    def run(self, instructions):
        self.stopped = False

        while not self.stopped:
            if not 0 <= self.pc < len(instructions):
                break
            next_instruction = instructions[self.pc]
            next_instruction(self)

instructions_builders = {
    'set': instruction(i_set),
    'sub': instruction(i_sub),
    'mul': instruction(i_mul),
    'jnz': instruction(i_jnz),
}

def parse_instruction(instruction_str):
    mnemonic, *args = instruction_str.split()
    make_instruction = instructions_builders[mnemonic]
    return make_instruction(*args)

def day_23_1(instructions):
    program = Program()
    program.run(instructions)

    return program.mul_count

def day_23_2(instructions):
    # Non trivially solvable without human analysis on the input...
    return len([
        x for x in range(107900, 124901, 17)
        if any(x % n == 0 for n in range(2, x // 2))
    ])

instructions = [
    parse_instruction(instruction_str)
    for instruction_str in my_input.split('\n')
]

print(f'part 1: {day_23_1(instructions)}')
print(f'part 2: {day_23_2(instructions)}')
