const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day19.txt");

pub fn day19() -> (Value, Value) {
    let (pc_idx, instrs) = parse_input(RAW_INPUT);

    (part1(pc_idx, &instrs), part2(pc_idx, &instrs))
}

pub fn part1(pc_idx: usize, instructions: &[Instruction]) -> Value {
    let final_registers = execute(pc_idx, instructions, Registers::default())
        .last()
        .expect("Empty program");

    final_registers[0]
}

pub fn part2(pc_idx: usize, instructions: &[Instruction]) -> Value {
    let initial_registers = [1, 0, 0, 0, 0, 0];

    let registers_after_setup = execute(pc_idx, instructions, initial_registers)
        .skip(100)
        .next()
        .expect("Empty program");

    divisors(registers_after_setup[4]).sum()
}

fn execute(pc_idx: usize, instructions: &[Instruction], regs: Registers)
    -> impl Iterator<Item = Registers> + '_
{
    Device { pc: 0, pc_idx, regs, instructions }
}

struct Device<'a> {
    pc: usize,
    pc_idx: usize,
    regs: Registers,
    instructions: &'a [Instruction],
}

impl Iterator for Device<'_> {
    type Item = Registers;

    fn next(&mut self) -> Option<Self::Item> {
        self.regs[self.pc_idx] = self.pc;

        match self.instructions.get(self.pc) {
            None => None,
            Some(instr) => {
                let [a, b] = instr.inputs;
                self.regs[instr.output] = (instr.op)(self.regs, a, b);
                self.pc = self.regs[self.pc_idx] + 1;
                Some(self.regs)
            }
        }
    }
}

fn divisors(n: Value) -> impl Iterator<Item = Value> {
    use arrayvec::ArrayVec;

    (1..(n as f32).sqrt() as Value)
        .filter(move |x| n % x == 0)
        .flat_map(move |x| {
            let mut divisors = ArrayVec::<[_; 2]>::new();
            divisors.push(x);
            if n / x != x { divisors.push(n / x) }
            divisors
        })
}

pub fn parse_input(input: &[u8]) -> (usize, Vec<Instruction>) {
    let mut lines = input.split(|&c| c == b'\n');

    let pc_idx = usize::from(lines.next().unwrap()[4] - b'0');

    let instructions = lines
        .map(|line| {
            let mut tokens = line.split(|&c| c == b' ');

            let mnemonic = tokens.next().unwrap();
            let op: Op = match mnemonic {
                b"addr" => |regs, a, b| regs[a] + regs[b],
                b"addi" => |regs, a, b| regs[a] + b,
                b"mulr" => |regs, a, b| regs[a] * regs[b],
                b"muli" => |regs, a, b| regs[a] * b,
                b"banr" => |regs, a, b| regs[a] & regs[b],
                b"bani" => |regs, a, b| regs[a] & b,
                b"borr" => |regs, a, b| regs[a] | regs[b] ,
                b"bori" => |regs, a, b| regs[a] | b,
                b"setr" => |regs, a, _| regs[a],
                b"seti" => |_   , a, _| a,
                b"gtir" => |regs, a, b| if a > regs[b] { 1 } else { 0 },
                b"gtri" => |regs, a, b| if regs[a] > b { 1 } else { 0 },
                b"gtrr" => |regs, a, b| if regs[a] > regs[b] { 1 } else { 0 },
                b"eqir" => |regs, a, b| if a == regs[b] { 1 } else { 0 } ,
                b"eqri" => |regs, a, b| if regs[a] == b { 1 } else { 0 } ,
                b"eqrr" => |regs, a, b| if regs[a] == regs[b] { 1 } else { 0 },
                invalid => panic!("Invalid op mnemonic: {:?}", invalid)
            };

            let mut num_tokens = tokens
                .map(|tok| std::str::from_utf8(tok).unwrap())
                .map(|tok_s| tok_s.parse().unwrap());

            let a = num_tokens.next().unwrap();
            let b = num_tokens.next().unwrap();
            let c = num_tokens.next().unwrap();

            Instruction {
                op,
                inputs: [a, b],
                output: c,
            }
        })
        .collect();

    (pc_idx, instructions)
}

type Value = usize;
type Registers = [Value; 6];
type Op = fn(Registers, Value, Value) -> Value;

pub struct Instruction {
    op: Op,
    inputs: [Value; 2],
    output: usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (pc_idx, instrs) = parse_input(RAW_INPUT);

        assert_eq!(part1(pc_idx, &instrs), 1968);
    }

    #[test]
    fn p2() {
        let (pc_idx, instrs) = parse_input(RAW_INPUT);

        assert_eq!(part2(pc_idx, &instrs), 21211200);
    }
}
