extern crate nom;

fn get_seq_number(row: u32, col: u32) -> u32 {
    (1..=col+row-1).fold(0, |acc, x| acc + x) - row + 1
}

fn parse_row_col(input: &str) -> (u32, u32) {
    use self::nom::*;

    use std::str::from_utf8;

    named!(number<u32>, map!(
        digit,
        |s| from_utf8(s).unwrap().parse::<u32>().unwrap()
    ));

    named!(coordinates<(u32, u32)>, do_parse!(
        tag_s!("To continue, please consult the code grid in the manual.  Enter the code at row ") >>
        row: number         >>
        tag_s!(", column ") >>
        col: number         >>
        tag_s!(".")         >>
        ((row, col))
    ));

    match coordinates(input.as_bytes()) {
        IResult::Done(_, coords) => coords,
        _                        => panic!("Wrong manual format")
    }
}

fn code_at(index: u32) -> u64 {
    (2..=index).fold(20151125, |acc, _| (acc * 252533) % 33554393)
}

pub fn p1(input: &str) -> u64 {
    let (row, col) = parse_row_col(input.trim());

    code_at(get_seq_number(row, col))
}

pub fn p2(input: &str) -> u64 {
    42
}
