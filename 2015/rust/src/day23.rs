extern crate nom;

use std::collections::HashMap;

type Register = String;
type Offset = i32;

type Registers = HashMap<Register, u32>;

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

fn parse_instruction(raw_instruction: &str) -> Instruction {
    use self::nom::*;
    use self::Instruction::*;

    use std::str::from_utf8;

    named!(number<i32>, map!(
        digit,
        |s| from_utf8(s).unwrap().parse::<i32>().unwrap()
    ));

    named!(register<Register>, map!(
        alt!(tag_s!("a") | tag_s!("b")),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(offset<self::Offset>, do_parse!(
        sign: alt!(tag_s!("+") | tag_s!("-")) >>
        value: number                         >>
        (if sign == "+".as_bytes() { value } else { -value })
    ));

    named!(hlf<Instruction>, do_parse!(
        tag_s!("hlf ") >> reg: register >> (Hlf(reg))
    ));
    named!(tpl<Instruction>, do_parse!(
        tag_s!("tpl ") >> reg: register >> (Tpl(reg))
    ));
    named!(inc<Instruction>, do_parse!(
        tag_s!("inc ") >> reg: register >> (Inc(reg))
    ));
    named!(jmp<Instruction>, do_parse!(
        tag_s!("jmp ") >> off: offset   >> (Jmp(off))
    ));
    named!(jie<Instruction>, do_parse!(
        tag_s!("jie ") >> reg: register >>
        tag_s!(", ")   >> off: offset   >> (Jie(reg, off))
    ));
    named!(jio<Instruction>, do_parse!(
        tag_s!("jio ") >> reg: register >>
        tag_s!(", ")   >> off: offset   >> (Jio(reg, off))
    ));

    named!(instruction<Instruction>, alt!(hlf | tpl | inc | jmp | jie | jio));

    match instruction(raw_instruction.as_bytes()) {
        IResult::Done(_, instruction) => instruction,
        _                             => panic!("Wrong instruction format")
    }
}

fn update_reg(reg: &Register, regs: &mut Registers, f: fn(u32) -> u32) {
    let entry = regs.entry(reg.clone()).or_insert(0);
    *entry = f(*entry);
}

fn get_reg(reg: &Register, regs: &mut Registers) -> u32 {
    regs.get(reg).map_or(0, Clone::clone)
}

fn run(instructions: &Vec<Instruction>, regs: &mut Registers) {
    use self::Instruction::*;

    let mut pc = 0i32;

    loop {
        match instructions.get(pc as usize) {
            None => break,
            Some(instr) => {
                let mut pc_offset = 1;
                match *instr {
                    Hlf(ref r)    => update_reg(r, regs, |v| v / 2),
                    Tpl(ref r)    => update_reg(r, regs, |v| v * 3),
                    Inc(ref r)    => update_reg(r, regs, |v| v + 1),
                    Jmp(o)        => pc_offset = o,
                    Jie(ref r, o) => if get_reg(r, regs) % 2 == 0 { pc_offset = o; },
                    Jio(ref r, o) => if get_reg(r, regs) == 1     { pc_offset = o; },
                }
                pc += pc_offset;
            }
        }
    }
}

pub fn p1(input: &str) -> u32 {
    let instructions = input.trim().split('\n')
                                   .map(parse_instruction)
                                   .collect::<Vec<_>>();
    let mut registers = Registers::new();

    run(&instructions, &mut registers);

    registers.get(&"b".to_string()).map_or(0, Clone::clone)
}

pub fn p2(input: &str) -> u32 {
    let instructions = input.trim().split('\n')
                                   .map(parse_instruction)
                                   .collect::<Vec<_>>();
    let mut registers = Registers::new();
    registers.insert("a".to_string(), 1);

    run(&instructions, &mut registers);

    registers.get(&"b".to_string()).map_or(0, Clone::clone)
}
