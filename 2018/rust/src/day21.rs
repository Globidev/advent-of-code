const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day21.txt");

pub fn day21() -> (Value, Value) {
    let (pc_idx, instrs) = parse_input(RAW_INPUT);

    (part1(pc_idx, &instrs), part2(pc_idx, &instrs))
}

pub fn part1(pc_idx: usize, instructions: &[Instruction]) -> Value {
    use self::OpKind::EqRR;

    let (eq_instr, &cmp_reg) = instructions.iter()
        .enumerate()
        .find_map(|(idx, instr)| match instr {
            Instruction { op: (EqRR, _), inputs: [0, b], .. } => Some((idx, b)),
            Instruction { op: (EqRR, _), inputs: [a, 0], .. } => Some((idx, a)),
            _ => None
        })
        .expect("Input does not work with this logic");

    let mut device = Device {
        pc: 0,
        pc_idx,
        regs: Registers::default(),
        instructions
    };

    while device.pc != eq_instr {
        let _ = device.next();
    }

    device.regs[cmp_reg]
}

pub fn part2(pc_idx: usize, instructions: &[Instruction]) -> Value {
    use self::OpKind::{EqRR, AddI, GtRR};

    let (eq_instr, &cmp_reg) = instructions.iter()
        .enumerate()
        .find_map(|(idx, instr)| match instr {
            Instruction { op: (EqRR, _), inputs: [0, r], .. } => Some((idx, r)),
            Instruction { op: (EqRR, _), inputs: [r, 0], .. } => Some((idx, r)),
            _ => None
        })
        .expect("Input does not work with this logic");

    let (gt_instr, &gt_reg) = instructions.iter()
        .enumerate()
        .rev()
        .find_map(|(idx, instr)| match instr {
            Instruction { op: (GtRR, _), inputs: [_, r], .. } => Some((idx, r)),
            _ => None
        })
        .expect("Input does not work with this logic");

    let add_instr = gt_instr - 2;

    let &add_reg = match &instructions[add_instr] {
        Instruction { op: (AddI, _), inputs: [r, _], .. } => r,
        x => panic!("Input does not work with this logic: {:?}", x)
    };

    let mut device = Device {
        pc: 0,
        pc_idx,
        regs: Registers::default(),
        instructions
    };

    let mut seen = hashbrown::HashSet::new();
    let mut last_value = 0;

    loop {
        while device.pc != add_instr {
            let _ = device.next();
        }
        // Fast forward the inner loop
        device.regs[add_reg] = device.regs[gt_reg] / 256;

        while device.pc != eq_instr {
            let _ = device.next();
        }

        if !seen.insert(device.regs[cmp_reg]) {
            return last_value
        }

        last_value = device.regs[cmp_reg];
    }
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
                self.regs[instr.output] = (instr.op.1)(self.regs, a, b);
                self.pc = self.regs[self.pc_idx] + 1;
                Some(self.regs)
            }
        }
    }
}

pub fn parse_input(input: &[u8]) -> (usize, Vec<Instruction>) {
    let mut lines = input.split(|&c| c == b'\n');

    let pc_idx = usize::from(lines.next().unwrap()[4] - b'0');

    let instructions = lines
        .map(|line| {
            use self::OpKind::*;
            let mut tokens = line.split(|&c| c == b' ');

            let mnemonic = tokens.next().unwrap();
            let op: Op = match mnemonic {
                b"addr" => (AddR, |regs, a, b| regs[a] + regs[b]),
                b"addi" => (AddI, |regs, a, b| regs[a] + b),
                b"mulr" => (MulR, |regs, a, b| regs[a] * regs[b]),
                b"muli" => (MulI, |regs, a, b| regs[a] * b),
                b"banr" => (BanR, |regs, a, b| regs[a] & regs[b]),
                b"bani" => (BanI, |regs, a, b| regs[a] & b),
                b"borr" => (BorR, |regs, a, b| regs[a] | regs[b]),
                b"bori" => (BorI, |regs, a, b| regs[a] | b),
                b"setr" => (SetR, |regs, a, _| regs[a]),
                b"seti" => (SetI, |_   , a, _| a),
                b"gtir" => (GtIR, |regs, a, b| if a > regs[b] { 1 } else { 0 }),
                b"gtri" => (GtRI, |regs, a, b| if regs[a] > b { 1 } else { 0 }),
                b"gtrr" => (GtRR, |regs, a, b| if regs[a] > regs[b] { 1 } else { 0 }),
                b"eqir" => (EqIR, |regs, a, b| if a == regs[b] { 1 } else { 0 } ),
                b"eqri" => (EqRI, |regs, a, b| if regs[a] == b { 1 } else { 0 } ),
                b"eqrr" => (EqRR, |regs, a, b| if regs[a] == regs[b] { 1 } else { 0 }),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OpKind {
    AddR, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtIR, GtRI, GtRR,
    EqIR, EqRI, EqRR,
}

type Value = usize;
type Registers = [Value; 6];
type Op = (OpKind, fn(Registers, Value, Value) -> Value);

#[derive(Debug)]
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

        assert_eq!(part1(pc_idx, &instrs), 8797248);
    }

    #[test]
    fn p2() {
        let (pc_idx, instrs) = parse_input(RAW_INPUT);

        assert_eq!(part2(pc_idx, &instrs), 3007673);
    }
}
