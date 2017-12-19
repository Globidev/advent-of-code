from lib import readinput
my_input = readinput(__file__)

from collections import defaultdict
import asyncio

def instruction(f):

    def run(program, *tail_args):
        f(program, *tail_args)
        program.pc += 1

    return lambda *args: lambda p: run(p, *args)

def i_snd(program, x): program.play(program.value(x))
def i_set(program, x, y): program.regs[x] = program.value(y)
def i_add(program, x, y): program.regs[x] += program.value(y)
def i_mul(program, x, y): program.regs[x] *= program.value(y)
def i_mod(program, x, y): program.regs[x] %= program.value(y)

def i_rcv(program, x):
    if program.value(x) != 0:
        program.stop()

def i_jgz(program, x, y):
    if program.value(x) > 0:
        program.pc += program.value(y) - 1

class Program:

    def __init__(self):
        self.pc = 0
        self.regs = defaultdict(int)
        self.frequencies_played = []
        self.stopped = False

    def play(self, frequency):
        self.frequencies_played.append(frequency)

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

def parse_instruction(instruction_str, instructions_builders):
    mnemonic, *args = instruction_str.split()
    make_instruction = instructions_builders[mnemonic]
    return make_instruction(*args)

p1_instructions_builders = {
    'set': instruction(i_set),
    'add': instruction(i_add),
    'mul': instruction(i_mul),
    'mod': instruction(i_mod),
    'jgz': instruction(i_jgz),
    'snd': instruction(i_snd),
    'rcv': instruction(i_rcv),
}

def async_instruction_wrap(f):

    async def run(program, *tail_args):
        f(program, *tail_args)
        program.pc += 1

    return lambda *args: lambda p, *_: run(p, *args)

def async_instruction(f):

    async def run(program, *tail_args):
        await f(program, *tail_args)
        program.pc += 1

    return lambda *args: lambda *head: run(*head, *args)

async def i_snd_p2(program, _, out_queue, x):
    await program.send(out_queue, program.value(x))

async def i_rcv_p2(program, in_queue, out_queue, x):
    if in_queue.qsize() == 0:
        if out_queue.qsize() == 0:
            program.stop()
        else:
            await out_queue.join()
            program.pc -= 1
    else:
        program.regs[x] = await in_queue.get()
        in_queue.task_done()

p2_instructions_builders = {
    'set': async_instruction_wrap(i_set),
    'add': async_instruction_wrap(i_add),
    'mul': async_instruction_wrap(i_mul),
    'mod': async_instruction_wrap(i_mod),
    'jgz': async_instruction_wrap(i_jgz),
    'snd': async_instruction(i_snd_p2),
    'rcv': async_instruction(i_rcv_p2),
}

class PipedProgram(Program):

    def __init__(self, id):
        super().__init__()
        self.regs['p'] = id
        self.send_count = 0
        self.id = id

    async def run(self, instructions, in_queue, out_queue):
        self.stopped = False
        self.send_count = 0

        while not self.stopped:
            if not 0 <= self.pc < len(instructions):
                break
            next_instruction = instructions[self.pc]
            await next_instruction(self, in_queue, out_queue)

    async def send(self, queue, v):
        self.send_count += 1
        await queue.put(v)

def day_18_1(raw_instructions):
    instructions = [
        parse_instruction(instruction_str, p1_instructions_builders)
        for instruction_str in raw_instructions.split('\n')
    ]

    program = Program()
    program.run(instructions)

    return program.frequencies_played[-1]

def day_18_2(raw_instructions):
    instructions = [
        parse_instruction(instruction_str, p2_instructions_builders)
        for instruction_str in raw_instructions.split('\n')
    ]

    p0, p1 = PipedProgram(0), PipedProgram(1)
    q0, q1 = asyncio.Queue(), asyncio.Queue()
    r0, r1 = p0.run(instructions, q0, q1), p1.run(instructions, q1, q0)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(asyncio.gather(r0, r1))

    return p1.send_count

print(f'part 1: {day_18_1(my_input)}')
print(f'part 2: {day_18_2(my_input)}')
