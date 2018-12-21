const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day16.txt");

pub fn day16() -> (usize, u16) {
    let (samples, instrs) = parse_input(RAW_INPUT);

    (part1(&samples), part2(&samples, &instrs))
}

pub fn part1(samples: &[InstructionSample]) -> usize {
    samples.iter()
        .filter(|sample| valid_op_codes_for_sample(sample) >= 3)
        .count()
}

use packed_simd::u8x64;

const ZEROS: u8x64 = u8x64::splat(0);
const ONES: u8x64 = u8x64::splat(1);
const THREES: u8x64 = u8x64::splat(3);

pub fn part1_vectorized(samples: &PackedSamples) -> usize {
    (0..samples.ias.len()).step_by(u8x64::lanes())
        .map(|idx| {
            let ias = u8x64::from_slice_unaligned(&samples.ias[idx..]);
            let ibs = u8x64::from_slice_unaligned(&samples.ibs[idx..]);
            let ras = u8x64::from_slice_unaligned(&samples.ras[idx..]);
            let rbs = u8x64::from_slice_unaligned(&samples.rbs[idx..]);
            let expecteds = u8x64::from_slice_unaligned(&samples.expecteds[idx..]);
            let test = |vec: u8x64| vec.eq(expecteds).select(ONES, ZEROS);

            let sums =
                test(ras + rbs) +
                test(ras + ibs) +
                test(ras * rbs) +
                test(ras * ibs) +
                test(ras & rbs) +
                test(ras & ibs) +
                test(ras | rbs) +
                test(ras | ibs) +
                test(ras) +
                test(ias) +
                test(ias.gt(rbs).select(ONES, ZEROS)) +
                test(ras.gt(ibs).select(ONES, ZEROS)) +
                test(ras.gt(rbs).select(ONES, ZEROS)) +
                test(ias.eq(rbs).select(ONES, ZEROS)) +
                test(ras.eq(ibs).select(ONES, ZEROS)) +
                test(ras.eq(rbs).select(ONES, ZEROS));

            sums.ge(THREES).select(ONES, ZEROS).wrapping_sum() as usize
        })
        .sum()
}

pub fn part2(samples: &[InstructionSample], program: &[UnknownInstruction]) -> u16 {
    let mut op_masks = [u16::max_value(); OP_COUNT];
    let mut ops = [Op::AddI; OP_COUNT];
    let mut found = 0;

    'outer: for sample in samples {
        for op in &ALL_OPS {
            let instr = sample.instruction.with_op(*op);
            let mask = 1 << sample.instruction.op_code;
            if execute(instr, sample.before) != sample.after {
                op_masks[*op as usize] &= !mask;
                let op_mask = op_masks[*op as usize];
                if op_mask.count_ones() == 1 {

                    for i in 0..16 {
                        if op_mask & (1 << i) == op_mask {
                            ops[i] = *op;
                            found += 1;

                            if found == OP_COUNT {
                                break 'outer;
                            }

                            for m in &mut op_masks {
                                *m &= !(1 << i);
                            }

                            break
                        }
                    }


                }
            }
        }
    }

    let final_state = program.iter()
        .fold(Registers::default(), |regs, unknown_instr| {
            let op = ops[unknown_instr.op_code as usize];
            execute(unknown_instr.with_op(op), regs)
        });

    final_state[0]
}

fn valid_op_codes_for_sample(sample: &InstructionSample) -> u8 {
    let [ia, ib] = sample.instruction.inputs;
    let ra = sample.before[ia as usize];
    let rb = sample.before[ib as usize];
    let expected = sample.after[sample.instruction.output as usize];
    let test = |result| (result == expected) as u8;

    test(ra + rb) +
    test(ra + ib) +
    test(ra * rb) +
    test(ra * ib) +
    test(ra & rb) +
    test(ra & ib) +
    test(ra | rb) +
    test(ra | ib) +
    test(ra) +
    test(ia) +
    test((ia >  rb) as Value) +
    test((ra >  ib) as Value) +
    test((ra >  rb) as Value) +
    test((ia == rb) as Value) +
    test((ra == ib) as Value) +
    test((ra == rb) as Value)
}

#[derive(Debug, Default)]
pub struct PackedSamples {
    ias: Vec<u8>,
    ibs: Vec<u8>,
    ras: Vec<u8>,
    rbs: Vec<u8>,
    expecteds: Vec<u8>,
}

pub fn vectorize(samples: &[InstructionSample]) -> PackedSamples {
    let mut packed = PackedSamples::default();

    samples.iter()
        .for_each(|sample| {
            let [ia, ib] = sample.instruction.inputs;
            let ra = sample.before[ia as usize];
            let rb = sample.before[ib as usize];
            let expected = sample.after[sample.instruction.output as usize];

            packed.ias.push(ia as u8);
            packed.ibs.push(ib as u8);
            packed.ras.push(ra as u8);
            packed.rbs.push(rb as u8);
            packed.expecteds.push(expected as u8);
        });

    while packed.ias.len() % u8x64::lanes() != 0 {
        packed.ias.push(0);
        packed.ibs.push(0);
        packed.ras.push(0);
        packed.rbs.push(0);
        packed.expecteds.push(u8::max_value());
    }

    packed
}

fn execute(instr: Instruction, mut regs: Registers) -> Registers {
    let [a, b] = instr.inputs;

    let reg = |x| regs[x as usize];

    let res = match instr.op {
        AddR => { reg(a) + reg(b) },
        AddI => { reg(a) + b },
        MulR => { reg(a) * reg(b) },
        MulI => { reg(a) * b },
        BanR => { reg(a) & reg(b) },
        BanI => { reg(a) & b },
        BorR => { reg(a) | reg(b)  },
        BorI => { reg(a) | b },
        SetR => { reg(a) },
        SetI => { a },
        GtIR => { if a > reg(b) { 1 } else { 0 } },
        GtRI => { if reg(a) > b { 1 } else { 0 } },
        GtRR => { if reg(a) > reg(b) { 1 } else { 0 } },
        EqIR => { if a == reg(b) { 1 } else { 0 }  },
        EqRI => { if reg(a) == b { 1 } else { 0 }  },
        EqRR => { if reg(a) == reg(b) { 1 } else { 0 } },
    };

    regs[instr.output as usize] = res;

    regs
}

pub fn parse_input(input: &[u8]) -> (Vec<InstructionSample>, Vec<UnknownInstruction>) {
    let parse_registers = |line: &[u8]| [
        digit(line[09]),
        digit(line[12]),
        digit(line[15]),
        digit(line[18]),
    ];

    let parse_raw_instruction = |line: &[u8]| {
        let mut i = 0;
        let mut op_code = digit(line[0]);
        if line[1] != b' ' {
            op_code = op_code * 10 + digit::<u8>(line[1]);
            i = 1;
        }

        UnknownInstruction {
            op_code,
            inputs: [digit(line[i+2]), digit(line[i+4])],
            output: digit(line[i+6]),
        }
    };

    let mut lines = input.split(|&c| c == b'\n');
    let mut samples = Vec::with_capacity(2048);

    loop {
        let before = lines.next().unwrap();

        if !before.starts_with(b"Before") {
            break
        }

        let raw_instr = lines.next().unwrap();
        let after = lines.next().unwrap();
        let _empty = lines.next();

        let sample = InstructionSample {
            before: parse_registers(before),
            after: parse_registers(after),
            instruction: parse_raw_instruction(raw_instr)
        };

        samples.push(sample);
    }

    let instrs = lines.skip(1)
        .map(parse_raw_instruction)
        .collect();

    (samples, instrs)
}

use self::Op::*;

fn digit<T: From<u8>>(b: u8) -> T { T::from(b - b'0') }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    AddR = 0, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtIR, GtRI, GtRR,
    EqIR, EqRI, EqRR,
}

const OP_COUNT: usize = 16;
const ALL_OPS: [Op; OP_COUNT] = [
    AddR, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtIR, GtRI, GtRR,
    EqIR, EqRI, EqRR,
];

type Register = u16;
type Value = u16;
type Registers = [Register; 4];

#[derive(Debug)]
pub struct InstructionSample {
    before: Registers,
    after: Registers,
    instruction: UnknownInstruction,
}

#[derive(Debug)]
pub struct UnknownInstruction {
    op_code: u8,
    inputs: [Value; 2],
    output: u8
}

struct Instruction {
    op: Op,
    inputs: [Value; 2],
    output: u8
}

impl UnknownInstruction {
    fn with_op(&self, op: Op) -> Instruction {
        Instruction {
            op,
            inputs: self.inputs,
            output: self.output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (samples, _) = parse_input(RAW_INPUT);

        assert_eq!(part1(&samples), 542);
    }

    #[test]
    fn p1_vectorized() {
        let (samples, _) = parse_input(RAW_INPUT);
        let vectorized = vectorize(&samples);

        assert_eq!(part1_vectorized(&vectorized), 542);
    }

    #[test]
    fn p2() {
        let (samples, instrs) = parse_input(RAW_INPUT);

        assert_eq!(part2(&samples, &instrs), 575);
    }
}
