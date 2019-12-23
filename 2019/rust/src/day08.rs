use std::fmt::{self, Debug};
use itertools::Itertools;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day08.txt");
const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;
const IMG_PX_COUNT: usize = IMG_WIDTH * IMG_HEIGHT;

pub fn day08() -> impl Debug {
    let image_data = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&image_data), Banner(part2(&image_data)))
}

pub fn part1(image_data: &[u8]) -> usize {
    let layer_with_fewest_0s = layers(image_data)
        .min_by_key(|layer| layer.iter().filter(|&&b| b == 0).count())
        .expect("No layer");

    let (ones, twos) = layer_with_fewest_0s.iter()
        .fold((0, 0), |(ones, twos), &b| (
            ones + usize::from(b == 1),
            twos + usize::from(b == 2)
        ));

    ones * twos
}

pub fn part2(image_data: &[u8]) -> String {
    let colored_layers = layers(image_data)
        .map(|layer| layer.iter().map(|&x| Color::from(x)));

    let mut image = [Color::Transparent; IMG_PX_COUNT];

    for layer in colored_layers {
        for (pixel, color) in Iterator::zip(image.iter_mut(), layer) {
            *pixel = pixel.combine(color);
        }
    }

    image.chunks(IMG_WIDTH).into_iter()
        .map(|image_row|
            image_row.iter()
                .map(|color| match color {
                    Color::Black => '░',
                    Color::Red => '▓',
                    Color::Transparent => ' ',
                })
                .collect::<String>()
        )
        .join("\n")
}

fn layers(image_data: &[u8]) -> impl Iterator<Item = &[u8]> + '_ {
    (0..image_data.len())
        .step_by(IMG_PX_COUNT)
        .map(move |layer_start_idx| &image_data[layer_start_idx..][..IMG_PX_COUNT])
}

pub struct Banner(String);

impl Debug for Banner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    Red,
    Transparent,
}

impl Color {
    fn combine(self, other: Color) -> Color {
        use Color::*;

        match self {
            Color::Black => Black,
            Color::Red => Red,
            Color::Transparent => other,
        }
    }
}

impl From<u8> for Color {
    fn from(d: u8) -> Self {
        use Color::*;

        match d {
            0 => Black,
            1 => Red,
            2 => Transparent,
            other => panic!("{} is not a valid color digit", other),
        }
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = u8> + '_ {
    input.bytes()
        .map(|ascii_digit| ascii_digit - b'0')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part1(&input), 1_596);
    }

    #[test]
    fn p2() {
        let input = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&input), "\
▓░░░░▓▓▓░░▓▓▓░░░▓▓░░▓▓▓▓░
▓░░░░▓░░▓░▓░░▓░▓░░▓░▓░░░░
▓░░░░▓▓▓░░▓░░▓░▓░░░░▓▓▓░░
▓░░░░▓░░▓░▓▓▓░░▓░░░░▓░░░░
▓░░░░▓░░▓░▓░▓░░▓░░▓░▓░░░░
▓▓▓▓░▓▓▓░░▓░░▓░░▓▓░░▓▓▓▓░");
    }
}
