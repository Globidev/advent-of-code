extern crate nom;

#[derive(Clone, Copy, PartialEq)]
enum CandleStateP1 { On, Off }
type CandleStateP2 = u32;

struct Coord { x: u32, y: u32 }
struct Range { from: Coord, to: Coord }

enum Instruction {
    TurnOff(Range),
    TurnOn(Range),
    Toggle(Range),
}

const GRID_SIZE: usize = 1000 * 1000;
type CandleGrid<CandleState> = [CandleState; GRID_SIZE];
type CandleTranform<CandleState> = fn(CandleState) -> CandleState;
type Apply<CandleState> = fn(&mut CandleGrid<CandleState>, Instruction);

type P1Grid = CandleGrid<CandleStateP1>;
type P2Grid = CandleGrid<CandleStateP2>;
type P1Transform = CandleTranform<CandleStateP1>;
type P2Transform = CandleTranform<CandleStateP2>;

fn alter_grid<State: Copy>(
    grid: &mut CandleGrid<State>,
    range: Range,
    transform: CandleTranform<State>
) {
    for x in range.from.x..=range.to.x {
        for y in range.from.y..=range.to.y {
            let idx = (y * 1000 + x) as usize;
            grid[idx] = transform(grid[idx]);
        }
    }
}

fn apply_instruction_p1(grid: &mut P1Grid, instruction: Instruction) {
    use self::CandleStateP1::{On, Off};
    use self::Instruction::{TurnOff, TurnOn, Toggle};

    let (range, action): (_, P1Transform) = match instruction {
        TurnOff(range) => (range, |_| Off),
        TurnOn(range)  => (range, |_| On),
        Toggle(range)  => (range, |c| if c == On { Off } else { On }),
    };

    alter_grid(grid, range, action)
}

fn apply_instruction_p2(grid: &mut P2Grid, instruction: Instruction) {
    use self::Instruction::{TurnOff, TurnOn, Toggle};

    let (range, action): (_, P2Transform) = match instruction {
        TurnOff(range) => (range, |b| if b == 0 { 0 } else { b - 1 }),
        TurnOn(range)  => (range, |b| b + 1),
        Toggle(range)  => (range, |b| b + 2),
    };

    alter_grid(grid, range, action)
}

fn parse_instruction(raw_instr: &str) -> Instruction {
    use self::nom::*;
    use std::str::FromStr;
    use std::str::from_utf8;

    type InstrBuilder = fn(Range) -> Instruction;

    named!(prefixes<InstrBuilder>, alt!(
        map!(tag!("turn off "), |_| Instruction::TurnOff as InstrBuilder) |
        map!(tag!("turn on "),  |_| Instruction::TurnOn  as InstrBuilder) |
        map!(tag!("toggle "),   |_| Instruction::Toggle  as InstrBuilder)
    ));
    named!(number<u32>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap()
    ));
    named!(coord<Coord>, do_parse!(
        x: number >>
        char!(',') >>
        y: number >>
        (Coord { x: x, y: y })
    ));
    named!(instruction<Instruction>, do_parse!(
        prefix: prefixes    >>
        from:   coord       >>
        tag_s!(" through ") >>
        to:     coord       >>
        (prefix(Range { from: from, to: to }))
    ));

    match instruction(raw_instr.as_bytes()) {
        IResult::Done(_, instr) => instr,
        _                       => panic!("Wrong instruction format")
    }
}

fn run_part<R, State: Copy>(
    input: &str,
    state: State,
    apply_instr: Apply<State>,
    transform: fn(CandleGrid<State>) -> R
) -> R {
    let mut grid: CandleGrid<State> = [state; GRID_SIZE];
    let instructions = input.trim().split('\n')
                                   .map(parse_instruction);

    for instr in instructions {
        apply_instr(&mut grid, instr)
    }

    transform(grid)
}

pub fn p1(input: &str) -> u32 {
    run_part(input, CandleStateP1::Off, apply_instruction_p1, |grid|
        grid.iter().filter(|cs| **cs == CandleStateP1::On).count() as u32
    )
}

pub fn p2(input: &str) -> u32 {
    run_part(input, 0u32, apply_instruction_p2, |grid|
        grid.iter().sum()
    )
}
