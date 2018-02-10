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

    named!(hlf<Instruction>, do_parse!(tag_s!("hlf ") >> reg: register >> (Hlf(reg))));
    named!(tpl<Instruction>, do_parse!(tag_s!("tpl ") >> reg: register >> (Tpl(reg))));
    named!(inc<Instruction>, do_parse!(tag_s!("inc ") >> reg: register >> (Inc(reg))));
    named!(jmp<Instruction>, do_parse!(tag_s!("jmp ") >> off: offset   >> (Jmp(off))));
    named!(jie<Instruction>, do_parse!(
        tag_s!("jie ") >>
        reg: register  >>
        tag_s!(", ")   >>
        off: offset    >>
        (Jie(reg, off))
    ));
    named!(jio<Instruction>, do_parse!(
        tag_s!("jio ") >>
        reg: register  >>
        tag_s!(", ")   >>
        off: offset    >>
        (Jio(reg, off))
    ));

    named!(instruction<Instruction>, alt!(hlf | tpl | inc | jmp | jie | jio));

    match instruction(raw_instruction.as_bytes()) {
        IResult::Done(_, instruction) => instruction,
        _                           => panic!("Wrong character format")
    }
}

fn run(instructions: &Vec<Instruction>, registers: &mut Registers) {
    use self::Instruction::*;

    let mut pc = 0i32;

    loop {
        match instructions.get(pc as usize) {
            None => break,
            Some(instr) => {
                match instr {
                    &Hlf(ref r)    => {
                        let entry = registers.entry(r.clone()).or_insert(0);
                        *entry = *entry / 2;
                    },
                    &Tpl(ref r)    => {
                        let entry = registers.entry(r.clone()).or_insert(0);
                        *entry = *entry * 3;
                    },
                    &Inc(ref r)    => {
                        let entry = registers.entry(r.clone()).or_insert(0);
                        *entry = *entry + 1;
                    },
                    &Jmp(ref o)    => {
                        pc += o - 1;
                    },
                    &Jie(ref r, o) => {
                        let entry = registers.entry(r.clone()).or_insert(0);
                        if *entry % 2 == 0 {
                            pc += o - 1;
                        }
                    },
                    &Jio(ref r, o) => {
                        let entry = registers.entry(r.clone()).or_insert(0);
                        if *entry == 1 {
                            pc += o - 1;
                        }
                    },
                }
                pc += 1;
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
